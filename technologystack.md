# 技術スタック

## フロントエンド
- **フレームワーク**: React 19.1.0 (TypeScript)
- **ビルドツール**: Vite 7.0.4
- **UI**: CSS Modules / Styled Components
- **通知**: react-toastify 10.x
- **OBS通信**: obs-websocket-js 5.x

## バックエンド (Rust)
- **フレームワーク**: Tauri v2.x
- **非同期ランタイム**: tokio 1.x (full features)
- **WebSocket**: tokio-tungstenite 0.21.x
- **シリアライゼーション**: serde 1.x, serde_json 1.x
- **非同期処理**: futures 0.3.x

## プロトコル
- **OBS WebSocket**: v5.x (OBS Studio 28.x以降)
- **Master-Slave通信**: カスタムJSON over WebSocket

## 開発ツール
- **TypeScript**: ~5.8.3
- **Node.js**: 推奨 LTS版
- **Rust**: 1.70以降
- **パッケージマネージャー**: npm

## 対応プラットフォーム
- Windows
- macOS
- Linux

## アーキテクチャ
### Master モード
- OBSの変更を監視
- 変更をSlaveノードにブロードキャスト
- WebSocketサーバーとして動作

### Slave モード
- Masterからの変更を受信
- 受信した変更をローカルOBSに適用
- 非同期検出とアラート機能
- WebSocketクライアントとして動作

## 同期対象
- ソース（Source）
- プレビュー（Preview）
- プログラム（Program/Live Output）

## 主要機能
1. リアルタイム画像同期（内容、サイズ、位置）
2. Master-Slaveアーキテクチャ
3. 非同期アラート
4. 同期対象選択機能
5. LAN内自動検出（オプション）
