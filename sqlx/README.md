# コードリーディング
## Overview
Tokio, async-std をサポート
actix は runtime-tokio のalias

複数のfeatureが有効で、current thread で Tokio の Context が存在している時、Tokio runtime が使われる
つまり、tokio::runtime::Handle::try_current() がOKになる場合、これがErrだと async-std が使われる
runtime feature が存在しない or 複数のruntime が存在する場合でもコンパイルエラーにはならないが、
少なくとも１つのruntime feature を含んでいないと async 関数を呼び出した時にpanicになる

SQLiteはruntime を必要としないが、SqlitePool はタイムアウトと内部管理タスクのスポーンのためにruntimeのサポートを必要とする

### TLS support
#### tls-native-tls
native-tls backend を有効化する。これは OS-nativeなTLS機能を使用する
- macOS: SecureTransport
- Windows: SChannel
- other: OpenSSL

#### tls-results
RusTLS backend を有効化する。これはクロスプラットフォームなTLSライブラリ

両方のfeatureを用いている場合は tls-native-tls が優先される。
HandshakeFailure が出た場合は tls-native-tls を有効化すれば良い => tls-results より tls-native-tlsが優先されるため

## modules
### any
driverが実行時にリストの中から選ばれる。リストは install_drivers によりセットされる。
install_drivers は AnyConnection, AnyPoolがコネクトする前に必ず呼ばれなければならない。
全ての現在コンパイルされている driver を有効化するために、install_default_drivers を利用することが推奨される。
    => 最初に呼ばれた時だけ install する。2回目以降はなんの効果もない
    => このメソッドを介さずに install_drivers を呼び出すのは panic の原因になる

#### Structs
##### Any
無名のdriver。実際に使用される driver はコネクションURLから実行時に判断される。
- 実装 trait
    => Database, HasValueRef, HasArguments, HasStatement, HasStatementCache




## macros
## structs
### Query<'q, DB: Database, A>
- メンバ
    => pub(crate) statement: Either<&'q str, &'q <DB as HasStatement<'q>>::Statement>,
    => pub(crate) arguments: Option<A>,
    => pub(crate) database: PhantomData<DB>,
    => pub(crate) persistent: bool,
- trait
    => Execute
- メソッド
    # Execute の実装
    => fn sql(&self) -> &'q str;
        : 実行されるSQLを返す
    => fn statement(&self) -> Option<&<DB as HasStatement<'q>>::Statement>;
        : もし可能な場合は、事前にキャッシュしたstatementを返す
    => fn take_arguments(&mut self) -> Option<<DB as HasArguments<'q>>::Arguments>;
        : クエリ文字列に対してバインドされる Argumentを返す
    => fn persistent(&self) -> bool
        : trueの場合はstatementをキャッシュする
    # 個別実装
    => pub fn bind<T: 'q + Send + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self
        : 値をSQLにバインドする
    => pub fn persistent(mut self, value: bool) -> Self
        : trueに設定すると、statementがキャッシュされる: Default: true
    => pub fn map<F, O>(
            self,
            mut f: F,
        ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<O, Error> + Send, A>
        where
            F: FnMut(DB::Row) -> O + Send,
            O: Unpin,
        : 結果の各行を他の型に map する
    => pub fn try_map<F, O>(self, f: F) -> Map<'q, DB, F, A>
        where
            F: FnMut(DB::Row) -> Result<O, Error> + Send,
            O: Unpin,
        : 結果の各行を他の型に map する。 tryバージョン
    => pub async fn execute<'e, 'c: 'e, E>(self, executor: E) -> Result<DB::QueryResult, Error>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : クエリを実行し、row affected の行数を返す
    => pub async fn execute_many<'e, 'c: 'e, E>(
            self,
            executor: E,
        ) -> BoxStream<'e, Result<DB::QueryResult, Error>>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : 複数のクエリを実行し、各クエリで影響を受けた行数をストリームで返す
    => pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<DB::Row, Error>>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : クエリを実行し、結果をストリームで返す
    => pub fn fetch_many<'e, 'c: 'e, E>(
            self,
            executor: E,
        ) -> BoxStream<'e, Result<Either<DB::QueryResult, DB::Row>, Error>>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : 複数のクエリを実行し、結果をストリームで返す
    => pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<DB::Row>, Error>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : クエリを実行し、全ての結果をVecにして返す
    => pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<DB::Row, Error>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : クエリを実行して、ちょうど１つの結果を返す
    => pub async fn fetch_optional<'e, 'c: 'e, E>(self, executor: E) -> Result<Option<DB::Row>, Error>
        where
            'q: 'e,
            A: 'e,
            E: Executor<'c, Database = DB>,
        : クエリを実行して、多くとも１つの行を返す

### Map<'q, DB: Database, F, A>
- メンバ
    => inner: Query<'q, DB, A>,
    => mapper: F,
- 実装
    # Execute の実装
    => fn sql(&self) -> &'q str;
        : 実行されるSQLを返す
    => fn statement(&self) -> Option<&<DB as HasStatement<'q>>::Statement>;
        : もし可能な場合は、事前にキャッシュしたstatementを返す
    => fn take_arguments(&mut self) -> Option<<DB as HasArguments<'q>>::Arguments>;
        : クエリ文字列に対してバインドされる Argumentを返す
    => fn persistent(&self) -> bool
        : trueの場合はstatementをキャッシュする
    # 個別の実装
    => pub fn map<G, P>(
            self,
            mut g: G,
        ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<P, Error> + Send, A>
        where
            G: FnMut(O) -> P + Send,
            P: Unpin,
        : 結果の各行を 別の型へ map する
    => pub fn try_map<G, P>(
            self,
            mut g: G,
        ) -> Map<'q, DB, impl FnMut(DB::Row) -> Result<P, Error> + Send, A>
        where
            G: FnMut(O) -> Result<P, Error> + Send,
            P: Unpin,
        : 結果の各行を 別の型へ map する。tryバージョン
    => pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、結果をストリームで返す
    => pub fn fetch_many<'e, 'c: 'e, E>(
            mut self,
            executor: E,
        ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : 複数のクエリを実行し、結果をストリームで返す
    => pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、全ての結果をVecにして返す
    => pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、ちょうど１つの結果を返す
    => pub async fn fetch_optional<'e, 'c: 'e, E>(mut self, executor: E) -> Result<Option<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、多くとも１つの行を返す

### QueryAs<'q, DB: Database, Q, A>
生のSQLをパラメータにバインドして実行し、結果をFromRowの具象型にマップして返す

- メンバ
    => pub(crate) inner: Query<'q, DB, A>,
    => pub(crate) output: PhantomData<O>,

- メソッド
    # Execute の実装
    => fn sql(&self) -> &'q str;
        : 実行されるSQLを返す
    => fn statement(&self) -> Option<&<DB as HasStatement<'q>>::Statement>;
        : もし可能な場合は、事前にキャッシュしたstatementを返す
    => fn take_arguments(&mut self) -> Option<<DB as HasArguments<'q>>::Arguments>;
        : クエリ文字列に対してバインドされる Argumentを返す
    => fn persistent(&self) -> bool
        : trueの場合はstatementをキャッシュする
    # 個別実装
    => pub fn bind<T: 'q + Send + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self
        : 値をSQLにバインドする
    => pub fn persistent(mut self, value: bool) -> Self
        : trueに設定すると、statementがキャッシュされる: Default: true
    => pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、結果をストリームで返す
    => pub fn fetch_many<'e, 'c: 'e, E>(
            mut self,
            executor: E,
        ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : 複数のクエリを実行し、結果をストリームで返す
    => pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、全ての結果をVecにして返す
    => pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、ちょうど１つの結果を返す
    => pub async fn fetch_optional<'e, 'c: 'e, E>(mut self, executor: E) -> Result<Option<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、多くとも１つの行を返す

### QueryScalar<'q, DB: Database, O, A>
SQLを実行して、結果をFromRowの具象型にマップする

- メンバ
    => pub(crate) inner: QueryAs<'q, DB, (O,), A>,

- メソッド
    # Execute の実装
    => fn sql(&self) -> &'q str;
        : 実行されるSQLを返す
    => fn statement(&self) -> Option<&<DB as HasStatement<'q>>::Statement>;
        : もし可能な場合は、事前にキャッシュしたstatementを返す
    => fn take_arguments(&mut self) -> Option<<DB as HasArguments<'q>>::Arguments>;
        : クエリ文字列に対してバインドされる Argumentを返す
    => fn persistent(&self) -> bool
        : trueの場合はstatementをキャッシュする
    # 個別実装
    => pub fn bind<T: 'q + Send + Encode<'q, DB> + Type<DB>>(mut self, value: T) -> Self
        : 値をSQLにバインドする
    => pub fn persistent(mut self, value: bool) -> Self
        : trueに設定すると、statementがキャッシュされる: Default: true
    => pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、結果をストリームで返す
    => pub fn fetch_many<'e, 'c: 'e, E>(
            mut self,
            executor: E,
        ) -> BoxStream<'e, Result<Either<DB::QueryResult, O>, Error>>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : 複数のクエリを実行し、結果をストリームで返す
    => pub async fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> Result<Vec<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行し、全ての結果をVecにして返す
    => pub async fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> Result<O, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、ちょうど１つの結果を返す
    => pub async fn fetch_optional<'e, 'c: 'e, E>(mut self, executor: E) -> Result<Option<O>, Error>
        where
            'q: 'e,
            E: 'e + Executor<'c, Database = DB>,
            DB: 'e,
            F: 'e,
            O: 'e,
        : クエリを実行して、多くとも１つの行を返す
    
### Pool<DB: Database>(pub(crate) Arc<PoolInner<DB>>)


## traits
### Database
driverが実装しているトレイトの集合を内包する
- 関連型
    => Connection<Database = Self>, TransactionManager<Database = Self>, Row<Database = Self>
    => QueryResult: 'static + Sized + Send + Sync + Default + Extend<Self::QueryResult>
    => Column<Database = Self>, Value<Database = Self> + 'static
    => TypeInfo
- 定数
    => NAME, URL_SCHEMES

### HasValueRef<'r>
ライフタイムにおいて、Database と ValueRef を関連づけるためのトレイト
- 関連型
    => Database: Database
    => ValueRef: ValueRef<'r, Database = Self::Database>

### HasArguments<'q>
ライフタイムにおいて、Database と Arguments を関連づけるためのトレイト
- 関連型
    => Database: Database
    => Arguments: Arguments<'q, Database = Self::Database>;
    => ArgumentBuffer;

### HasStatement<'q>
ライフタイムにおいて、Database と Statement を関連づけるためのトレイト
- 関連型
    => Database: Database
    => Statement: Statement<'q, Database = Self::Database>

### HasStatementCache
prepared statement のキャッシュを保持する

### Connection
１つのコネクションを表す
- 関連型
    => Database: Database
    => Options: ConnectionOptions<Connection = Self>
- メソッド
    => close(self) -> BoxFuture<'static, Result<(), Error>>;
        : コネクションを閉じる => 明示的に閉じないとサーバ側がTCP keepaliveのタイムアウトになるまで気づかない
    => close_hard(self) -> BoxFuture<'static, Result<(), Error>>;
        : 即座にコネクションを閉じる => graceful shutdown ではない。TCP の FIN フレームによりサーバ側にこちらが死んだことを伝えられる
    => pping(&mut self) -> BoxFuture<'_, Result<(), Error>>;
        : コネクションがまだ有効化を問い合わせる
    => fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
        where
            Self: Sized;
        : 新しいトランザクションを開始するか / アクティブなトランザクション内でセーブポイントを確立する
    => fn transaction<'a, F, R, E>(&'a mut self, callback: F) -> BoxFuture<'a, Result<R, E>>
        where
            for<'c> F: FnOnce(&'c mut Transaction<'_, Self::Database>) -> BoxFuture<'c, Result<R, E>>
                + 'a
                + Send
                + Sync,
            Self: Sized,
            R: Send,
            E: From<Error> + Send,
        : トランザクション内で関数を実行する。beginからcommit or rollbackをこの関数内で完結する
    => fn cached_statements_size(&self) -> usize
        where
            Self::Database: HasStatementCache,
        : コネクション内で現在キャッシュされているステートメントの数を返す
    => fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>>
        where
            Self::Database: HasStatementCache,
        : キャッシ内の全ステートメントを削除する
    => fn shrink_buffers(&mut self);
        : メモリバッファを解放して、規定サイズのメモリ領域を新たに割り当ててもらう。大きなデータを扱うなどして不必要に大きくなったバッファを縮小するのに使う
    => fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>>;
    => fn should_flush(&self) -> bool;
    => fn connect(url: &str) -> BoxFuture<'static, Result<Self, Error>>
        where
            Self: Sized,
        : 新しいコネクションを確立する。オプション(Self::Options: ConnectionOptions<Connection = Self>)はURLをパースして渡す
    => fn connect_with(options: &Self::Options) -> BoxFuture<'_, Result<Self, Error>>
        : 新しいコネクションを確立する

### ConnectOptions
'static + Send + Sync + FromStr<Err = Error> + Debug + Clone
を実装している必要がある
- 関連型
    => Connection: Connection + ?Sized
- メソッド
    => fn from_url(url: &Url) -> Result<Self, Error>;
        : URLからConnectionOptionsをパースして作成する
    => fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
        where
            Self::Connection: Sized;
        : 新しいコネクションを確立する => Connection::connect_with は結局ここに移譲している
    => fn log_statements(self, level: LevelFilter) -> Self;
        : ログステーメントのレベルを設定する
    => fn log_slow_statements(self, level: LevelFilter, duration: Duration) -> Self;
        : クロークエリのログレベルを設定する
    => fn disable_statement_logging(self) -> Self
        : ログ、スロークエリログの両方をオフにする

### Transaction<'c, DB>
where
    DB: Database
- フィールド
    => connection: MaybePoolConnection<'c, DB>,
    => open: bool,
- メソッド
    => pub fn begin(
            conn: impl Into<MaybePoolConnection<'c, DB>>,
        ) -> BoxFuture<'c, Result<Self, Error>>
        : DB::TransactionManager::beginにコネクションを渡して呼ぶ。これでトランザクションを開始して、自分を返す(open = true)
    => pub async fn commit(mut self) -> Result<(), Error> 
        : DB::TransactionManager::commitにコネクションを渡して呼ぶ。 open = false にする
    => pub async fn rollback(mut self) -> Result<(), Error>
        : DB::TransactionManager::rollback にコネクションを渡して呼ぶ。 open = falseにする
- trait
    => 標準: Debug, Deref, DerefMut, AsMut, Drop
    => Acquire

### PoolConnection<DB: Database>
- メンバ
    => live: Option<Live<DB>>
    => pool: Arc<PoolInner<DB>>,

- trait
    => 標準: Debug, Deref, DerefMut, AsRef, Drop
    => Aquire

- メソッド
    => pub async fn close(mut self) -> Result<(), Error>
        : コネクションをクローズする。プールが別のコネクションを開けるようになる
    => pub fn detach(mut self) -> DB::Connection
        : コネクションをプールから切り離すが、閉じるわけではない。
    => pub fn leak(mut self) -> DB::Connection
        : コネクションをプールから切り離す。maximum capacity を1減らす。 pool の capacity に影響を与えたくない場合は detach を使う
    => fn take_live(&mut self) -> Live<DB>
        :self.live.take()
    => pub fn return_to_pool(&mut self) -> impl Future<Output = ()> + Send + 'static 
        : プールに戻す前にコネクションが生きていることを確認して、プールに返す。生きていない、返せなかった時は min_connectionsを更新する
    
TODO: Live, Idle, Floating, Pool

### TransactionManager
- 関連型
    => Dtabase: Database

- メソッド
    => fn begin(
            conn: &mut <Self::Database as Database>::Connection,
        ) -> BoxFuture<'_, Result<(), Error>>;
        : 新しいトランザクションを開始するか / アクティブなトランザクション内でセーブポイントを確立する
    => fn commit(
            conn: &mut <Self::Database as Database>::Connection,
        ) -> BoxFuture<'_, Result<(), Error>>;
        : アクティブなトランザクションをコミットする / most recent なセーブポイントをリリースする
    => fn rollback(
            conn: &mut <Self::Database as Database>::Connection,
        ) -> BoxFuture<'_, Result<(), Error>>;
        : アクティブなトランザクションをアボートする / most recent なセーブポイントを復旧する
    => fn start_rollback(conn: &mut <Self::Database as Database>::Connection);
        : アクティブなトランザクションをアボートを開始する / most recent なセーブポイントを復旧を開始する

### Row: Unpin + Send + Sync + 'static
- 関連型
    => Database: Database
- メソッド
    => fn is_empty(&self) -> bool
        : カラムがない場合 true
    => fn len(&self) -> usize
        : rowが持つカラムの数
    => fn column<I>(&self, index: I) -> &<Self::Database as Database>::Column
        where
            I: ColumnIndex<Self>,
        : indexを指定してカラム情報を取得する
    => fn try_column<I>(&self, index: I) -> Result<&<Self::Database as Database>::Column, Error>
        where
            I: ColumnIndex<Self>,
        : column の try バージョン
    => fn columns(&self) -> &[<Self::Database as Database>::Column];
        : 全てのカラムを取得する
    => fn get<'r, T, I>(&'r self, index: I) -> T
        where
            I: ColumnIndex<Self>,
            T: Decode<'r, Self::Database> + Type<Self::Database>,
        : カラムの値を取得する。カラムが存在しない場合 panic
    => fn get_unchecked<'r, T, I>(&'r self, index: I) -> T
        where
            I: ColumnIndex<Self>,
            T: Decode<'r, Self::Database>,
        : カラムの値を取得する。Rustの型と互換性があるかのチェックはしない。カラムが存在しない場合 panic
    => fn try_get<'r, T, I>(&'r self, index: I) -> Result<T, Error>
        where
            I: ColumnIndex<Self>,
            T: Decode<'r, Self::Database> + Type<Self::Database>,
        : カラムの値を取得する。 try バージョン
    => fn try_get_unchecked<'r, T, I>(&'r self, index: I) -> Result<T, Error>
        where
            I: ColumnIndex<Self>,
            T: Decode<'r, Self::Database>,
        : カラムの値を取得する。 try バージョン。Rustの型と互換性があるかのチェックはしない。
    => fn try_get_raw<I>(
            &self,
            index: I,
        ) -> Result<<Self::Database as HasValueRef<'_>>::ValueRef, Error>
        : カラムの値を取得する。値をデコードせずに、データベースの生の値のまま返す

### Column: 'static + Send + Sync + Debug
- 関連型
    => Database: Database
- メソッド
    => fn ordinal(&self) -> usize;
        : カラムの順序を取得する
    => fn name(&self) -> &str;
        : カラム名を取得する
    => fn type_info(&self) -> &<Self::Database as Database>::TypeInfo;
        : カラムの型情報を取得する

### TypeInfo: Debug + Display + Clone + PartialEq<Self> + Send + Sync 
- メソッド
    => fn is_null(&self) -> bool;
        : ColumnType::Nullと等しいか検査する
    => fn name(&self) -> &str;
        : データベースでのカラム型名を返す
    => fn is_void(&self) -> bool
        : PgType::Void と等しいか検査する。Pg以外はfalse

### Value
- 関連型
    => Database: Database
- メソッド
    => fn as_ref(&self) -> <Self::Database as HasValueRef<'_>>::ValueRef;
        : 値をreferenceとして取得する
    => fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo>;
        : 値の型情報を取得する
    => fn is_null(&self) -> bool;
        : NULL の時に trueを返す
    => fn decode<'r, T>(&'r self) -> T
        where
            T: Decode<'r, Self::Database> + Type<Self::Database>,
        : 値をデコードする
    => fn decode_unchecked<'r, T>(&'r self) -> T
        where
            T: Decode<'r, Self::Database>,
        : 値をデコードする。型の互換性は検査しない
    => fn try_decode<'r, T>(&'r self) -> Result<T, Error>
        where
            T: Decode<'r, Self::Database> + Type<Self::Database>,
        : 値をデコードする。tryバージョン
    => fn try_decode_unchecked<'r, T>(&'r self) -> Result<T, Error>
        where
            T: Decode<'r, Self::Database>
        : 値をデコードする。型の互換性は検査しない。tryバージョン

### ValueRef<'r>: Sized
- 関連型
    Database: Database
- メソッド
    => fn to_owned(&self) -> <Self::Database as Database>::Value;
        : referenceから実体を作り出す
    => fn type_info(&self) -> Cow<'_, <Self::Database as Database>::TypeInfo>;
        : 型情報を取得する
    => fn is_null(&self) -> bool;
        : NULL の場合に trueを返す

### Arguments<'q>: Send + Sized + Default
データベースに送るargument のタプル（byte、型情報のペア）

- 関連型
    => Database: Database
- メソッド
    => fn reserve(&mut self, additional: usize, size: usize);
        : キャパシティを予約する
    => fn add<T>(&mut self, value: T)
        where
            T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>;
        : argument のサイドに値を追加する
    => fn format_placeholder<W: Write>(&self, writer: &mut W) -> fmt::Result
        : プレースホルダのフォーマット

### Statement<'q>: Send + Sync
- 関連型
    => Database: Database
- メソッド
    => fn to_owned(&self) -> <Self::Database as HasStatement<'static>>::Statement;
        : statement をownedで作って返す。original のSQL文字列を返す
    => fn sql(&self) -> &str;
        : original のSQL文字列を返す
    => fn parameters(&self) -> Option<Either<&[<Self::Database as Database>::TypeInfo], usize>>;
        : statementに必要なパラメータを取得する。返ってくる情報はデータベース実装に依存する。Pgはタイプ情報をフルで返すが、SQLiteは数しか返さない
    => fn columns(&self) -> &[<Self::Database as Database>::Column];
        : statementを実行したときに返される予定のカラムを取得する
    => fn column<I>(&self, index: I) -> &<Self::Database as Database>::Column
        where
            I: ColumnIndex<Self>,
        : indexを指定してカラム情報を取得する
    => fn try_column<I>(&self, index: I) -> Result<&<Self::Database as Database>::Column, Error>
        where
            I: ColumnIndex<Self>,
        : indexを指定してからむ情報を取得する。tryバージョン
    => fn query(&self) -> Query<'_, Self::Database, <Self::Database as HasArguments<'_>>::Arguments>;

### Execute<'q, DB:Database>: Send + Sync
- メソッド
    => fn sql(&self) -> &'q str;
        : 実行されるSQLを返す
    => fn statement(&self) -> Option<&<DB as HasStatement<'q>>::Statement>;
        : もし可能な場合は、事前にキャッシュしたstatementを返す
    => fn take_arguments(&mut self) -> Option<<DB as HasArguments<'q>>::Arguments>;
        : クエリ文字列に対してバインドされる Argumentを返す
    => fn persistent(&self) -> bool
        : trueの場合はstatementをキャッシュする

### Executor<'c>: Send + Sized + Debug
- 関連型
    => Database: Database
- メソッド
    => fn execute<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxFuture<'e, Result<<Self::Database as Database>::QueryResult, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>,
        : クエリを実行して、影響を受けた行数を返す
    => fn execute_many<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxStream<'e, Result<<Self::Database as Database>::QueryResult, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>,
        : 複数のクエリを実行して、影響を受けた行数をストリームで返す
    => fn fetch<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxStream<'e, Result<<Self::Database as Database>::Row, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>,
        : クエリを実行して、結果をストリームで返す
    => fn fetch_many<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxStream<
            'e,
            Result<
                Either<<Self::Database as Database>::QueryResult, <Self::Database as Database>::Row>,
                Error,
            >,
        >
        where
            'c: 'e,
            E: Execute<'q, Self::Database>;
        : 複数のクエリを実行して、結果をストリームで返す
    => fn fetch_all<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxFuture<'e, Result<Vec<<Self::Database as Database>::Row>, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>,
        : クエリを実行して、全ての結果をVecで返す
    => fn fetch_one<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxFuture<'e, Result<<Self::Database as Database>::Row, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>,
        : クエリを実行して一行のみ返す
    => fn fetch_optional<'e, 'q: 'e, E: 'q>(
            self,
            query: E,
        ) -> BoxFuture<'e, Result<Option<<Self::Database as Database>::Row>, Error>>
        where
            'c: 'e,
            E: Execute<'q, Self::Database>;
        : クエリを実行して、多くとも一行返す
    => fn prepare<'e, 'q: 'e>(
            self,
            query: &'q str,
        ) -> BoxFuture<'e, Result<<Self::Database as HasStatement<'q>>::Statement, Error>>
        where
            'c: 'e,
        : パラメータと結果の型情報を検査するためのSQLを準備する
    => fn prepare_with<'e, 'q: 'e>(
            self,
            sql: &'q str,
            parameters: &'e [<Self::Database as Database>::TypeInfo],
        ) -> BoxFuture<'e, Result<<Self::Database as HasStatement<'q>>::Statement, Error>>
        where
            'c: 'e;
        : パラメータと結果の型情報を検査するためのSQLを準備する
    => fn describe<'e, 'q: 'e>(
            self,
            sql: &'q str,
        ) -> BoxFuture<'e, Result<Describe<Self::Database>, Error>>
        where
            'c: 'e;
        : パラメータ、戻り値の型情報を記述する

### Acquire<'c>
- 関連型
    => Database: Database
    => Connection: Deref<Target = <Self::Database as Database>::Connection> + DerefMut + Send;
- メソッド
    => fn acquire(self) -> BoxFuture<'c, Result<Self::Connection, Error>>;
    => fn begin(self) -> BoxFuture<'c, Result<Transaction<'c, Self::Database>, Error>>;

Poolは Acquireを実装
AnyConnection, MysqlConnection, PgConnection, SqliteConnection が実装している


## enums
### MaybePoolConnection<'c, DB: Database>
Connection(&c' mut DB::Connection),
PoolConnection(PoolConnection<DB>),

- trait
    => 標準: Deref, DerefMut, From<PoolConnection<DB>>, From<&'c mut DB::Connection>

## functions
## type definitions
## attribute macros
## derive macros
