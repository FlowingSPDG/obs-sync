# ディレクトリ構造

```
obs-sync/
├── src/                          # フロントエンドソース
│   ├── components/              # Reactコンポーネント
│   │   ├── MasterControl.tsx   # Masterモード制御UI
│   │   ├── SlaveMonitor.tsx    # Slaveモード監視UI
│   │   ├── SyncTargetSelector.tsx # 同期対象選択
│   │   ├── ConnectionStatus.tsx # 接続状態表示
│   │   ├── AlertPanel.tsx      # 非同期アラート表示
│   │   └── OBSSourceList.tsx   # OBSソース一覧
│   ├── hooks/                   # カスタムフック
│   │   ├── useOBSConnection.ts # OBS接続管理
│   │   ├── useSyncState.ts     # 同期状態管理
│   │   └── useNetworkStatus.ts # ネットワーク状態管理
│   ├── types/                   # TypeScript型定義
│   │   ├── obs.ts              # OBS関連型
│   │   ├── sync.ts             # 同期関連型
│   │   └── network.ts          # ネットワーク関連型
│   ├── utils/                   # ユーティリティ関数
│   │   ├── obsUtils.ts         # OBS操作ヘルパー
│   │   └── syncUtils.ts        # 同期処理ヘルパー
│   ├── styles/                  # スタイルシート
│   │   ├── global.css          # グローバルスタイル
│   │   └── components/         # コンポーネント別スタイル
│   ├── App.tsx                  # メインアプリケーション
│   ├── App.css                  # アプリケーションスタイル
│   └── main.tsx                 # エントリーポイント
│
├── src-tauri/                   # Tauriバックエンド
│   ├── src/
│   │   ├── main.rs             # メインエントリーポイント
│   │   ├── lib.rs              # ライブラリルート
│   │   ├── obs/                # OBS WebSocket統合
│   │   │   ├── mod.rs          # OBSモジュール
│   │   │   ├── client.rs       # OBS WebSocketクライアント
│   │   │   ├── events.rs       # OBSイベント処理
│   │   │   └── commands.rs     # OBSコマンド実行
│   │   ├── sync/               # 同期ロジック
│   │   │   ├── mod.rs          # 同期モジュール
│   │   │   ├── master.rs       # Masterモード実装
│   │   │   ├── slave.rs        # Slaveモード実装
│   │   │   ├── protocol.rs     # 通信プロトコル定義
│   │   │   └── diff.rs         # 差分検出ロジック
│   │   ├── network/            # ネットワーク通信
│   │   │   ├── mod.rs          # ネットワークモジュール
│   │   │   ├── server.rs       # WebSocketサーバー (Master用)
│   │   │   └── client.rs       # WebSocketクライアント (Slave用)
│   │   └── commands.rs         # Tauriコマンド定義
│   ├── Cargo.toml              # Rust依存関係
│   └── tauri.conf.json         # Tauri設定
│
├── public/                      # 静的ファイル
├── .gitignore
├── package.json                 # Node.js依存関係
├── tsconfig.json                # TypeScript設定
├── vite.config.ts               # Vite設定
├── README.md                    # プロジェクト説明
├── technologystack.md           # 技術スタック詳細
└── directorystructure.md        # このファイル
```

## モジュール説明

### フロントエンド (src/)
- **components/**: UI コンポーネント
  - Master/Slave モード切替と制御UI
  - 同期状態の可視化
  - アラート表示パネル
  
- **hooks/**: React カスタムフック
  - OBS接続の管理と状態保持
  - 同期状態の管理
  - ネットワーク状態の監視

- **types/**: TypeScript 型定義
  - OBS WebSocketの型
  - 同期プロトコルの型
  - ネットワークメッセージの型

- **utils/**: ヘルパー関数
  - OBS操作の抽象化
  - 同期処理のユーティリティ

### バックエンド (src-tauri/src/)
- **obs/**: OBS WebSocket統合
  - OBS Studioへの接続管理
  - イベントの監視と処理
  - コマンドの実行

- **sync/**: 同期ロジック
  - Masterモード: 変更検出とブロードキャスト
  - Slaveモード: 変更受信と適用
  - 差分検出アルゴリズム

- **network/**: ネットワーク通信
  - WebSocketサーバー（Master）
  - WebSocketクライアント（Slave）
  - メッセージのルーティング

- **commands.rs**: フロントエンドから呼び出せるTauriコマンド

## データフロー

### Masterモード
```
OBS Studio → obs/client.rs → sync/master.rs → network/server.rs → Slave Nodes
```

### Slaveモード
```
Master Node → network/client.rs → sync/slave.rs → obs/client.rs → Local OBS Studio
```
