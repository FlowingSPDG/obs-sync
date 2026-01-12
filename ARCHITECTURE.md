# OBS Sync アーキテクチャ設計

## 画像同期の仕組み

### 問題点
OBSの画像ソースは、各マシンのローカルファイルシステム上のファイルパスを参照しています。単純にファイルパスを同期しても、Slaveマシンに同じファイルが存在しない場合、同期は失敗します。

### 解決方法: バイナリデータの転送

#### Master側の処理
1. OBSイベント監視で画像ソースの変更を検知
2. 画像ファイルのパスを取得
3. **ファイルシステムから画像のバイナリデータを読み取る**
4. `ImageUpdatePayload`にファイルパスとバイナリデータを含めて送信

```rust
pub struct ImageUpdatePayload {
    pub scene_name: String,
    pub source_name: String,
    pub file: String,              // 元のファイルパス（参考用）
    pub image_data: Vec<u8>,       // 画像のバイナリデータ
    pub width: Option<f64>,
    pub height: Option<f64>,
}
```

#### Slave側の処理
1. `ImageUpdatePayload`を受信
2. バイナリデータを一時ディレクトリに保存（`%TEMP%/obs-sync/`）
3. 保存したファイルパスでOBSの画像ソースを更新

```
Master OBS           Slave OBS
  |                     |
  | 画像変更検知         |
  | → ファイル読み取り    |
  | → バイナリ送信  ---> | 受信
  |                     | → 一時ディレクトリに保存
  |                     | → OBS画像ソース更新
  |                     | → 画像表示✓
```

### メリット
- ✅ Slaveマシンに元の画像ファイルが存在しなくても同期可能
- ✅ 異なるOS/ファイルシステムでも動作
- ✅ ネットワーク越しでも完全な同期が可能

### デメリットと対策
- ⚠️ 大きな画像ファイルは転送に時間がかかる
  - 対策: 必要に応じて画像を圧縮
  - 対策: 差分検出（同じ画像は再送しない）
- ⚠️ ネットワーク帯域を使用
  - 対策: LAN内での使用を想定（高速）

## 初期状態同期（再接続時の同期ズレ回避）

### 問題点
Slaveが一度切断して再接続した場合、その間のMasterの変更を受信できず、同期ズレが発生します。

### 解決方法: StateSync メッセージ

#### 接続時の処理フロー

```
Slave接続
  ↓
Master: 現在の状態をスナップショット
  - 現在のプログラムシーン
  - 現在のプレビューシーン
  - 全シーンの情報
    - 各アイテムの位置・サイズ
    - 画像ソースの場合は画像データも含む
  ↓
StateSyncメッセージ送信
  ↓
Slave: 受信した状態を完全適用
  - シーン切り替え
  - 画像ソース更新
  - トランスフォーム設定
  ↓
同期完了✓
```

#### データ構造

```rust
pub struct StateSyncPayload {
    pub current_program_scene: String,
    pub current_preview_scene: Option<String>,
    pub scenes: Vec<SceneInfo>,
}

pub struct SceneInfo {
    pub scene_name: String,
    pub items: Vec<SceneItemInfo>,
}

pub struct SceneItemInfo {
    pub item_id: i64,
    pub source_name: String,
    pub source_type: String,
    pub transform: TransformData,
    pub image_path: Option<String>,
    pub image_data: Option<Vec<u8>>,  // 画像ソースの場合のみ
}
```

### 実装状態

#### ✅ 実装済み
- `StateSyncPayload`の定義
- Slave側の`apply_initial_state`処理
- Master側の`send_initial_state`メソッド
- 画像バイナリデータの転送仕組み

#### 🔄 今後の実装
- Master側で新しいSlaveが接続したときに`send_initial_state`を自動的に呼び出す
- obwsライブラリの実際のAPIに合わせた詳細な実装
- シーン一覧・アイテム一覧の取得
- トランスフォーム情報の完全な同期

## セキュリティ考慮事項

### ファイルパスの検証
- Masterから送信されたファイルパスは信頼せず、バイナリデータを使用
- Slave側では一時ディレクトリにのみ保存

### 一時ファイルの管理
- `%TEMP%/obs-sync/`に保存
- アプリケーション終了時にクリーンアップ（推奨）

## パフォーマンス最適化（将来）

### 画像のハッシュ管理
```rust
pub struct ImageUpdatePayload {
    pub hash: String,           // SHA-256ハッシュ
    pub image_data: Vec<u8>,
}
```

Slaveが既に同じハッシュの画像を持っている場合、転送をスキップ

### 画像圧縮
大きな画像は転送前に圧縮（JPEG品質調整、WebP変換など）

### 差分同期
初回接続時は全て送信、2回目以降は差分のみ

## テスト項目

### 画像同期テスト
- [ ] Master側で画像を追加 → Slaveに反映されるか
- [ ] Master側で画像を変更 → Slaveに反映されるか
- [ ] 大きな画像（10MB以上）の同期
- [ ] 複数の画像を連続で変更した場合

### 再接続テスト
- [ ] Slaveを切断→再接続 → 現在の状態に同期されるか
- [ ] Slaveが切断中にMasterで変更 → 再接続時に正しい状態になるか
- [ ] Master再起動後にSlaveが接続できるか

### エラーハンドリング
- [ ] 画像ファイルが存在しない場合
- [ ] 画像ファイルが破損している場合
- [ ] ネットワークエラー時の処理
- [ ] OBSが応答しない場合

## 開発メモ

### obwsライブラリのAPI調査が必要
以下の機能を実装するために、obwsライブラリの実際のAPIを確認する必要があります：

1. シーン一覧の取得
2. シーンアイテム一覧の取得
3. 画像ソースのファイルパス取得
4. 画像ソースのファイルパス設定
5. プレビューシーンの設定（スタジオモード）

### 参考
- obws: https://github.com/dnaka91/obws
- OBS WebSocket Protocol: https://github.com/obsproject/obs-websocket/blob/master/docs/generated/protocol.md
