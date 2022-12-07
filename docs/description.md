## Rawモードの有効化
ターミナルにはモードがあり、デフォルトではcanonicalモードになっています。
canonicalモードは行単位での入力となっており、enter(<LF>)などの入力を受け取ったタイミングで文字入力が完了となります。

しかし、エディタの入力は文字単位でなければなりません。
文字単位での入力にするにはrawモードにする必要があります。

まずcanonicalモードでの入力を試してみます。
https://github.com/AK-10/toy-editor/pull/1/commits/f3ffc49197ea053c1fdb9ff45cc4bc5c9d61fdbe
このようなコードを書いてみました

これを実行して雑に入力してみます

```
❯❯❯ cargo run
   Compiling toy-editor v0.1.0 (/home/ak-10/works/toy-editor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/toy-editor`
aaaaabbbbb
b: Ok(97)
b: Ok(97)
b: Ok(97)
b: Ok(97)
b: Ok(97)
b: Ok(98)
b: Ok(98)
b: Ok(98)
b: Ok(98)
b: Ok(98)
b: Ok(10)
```
キー入力を行い、エンターを押したタイミングでOk(97)のような出力がされました。
行単位の入力になっていることがわかります。

次にrawモードにした状態で入力してみます。
terminalはtermiosという構造体を利用して制御することができます。
これを利用してrawモードにしてみましょう。

流れとしては
- tcgetattrで現在のターミナルの状態を取得(termiosという構造体が得られます)
- 各フラグをrawモードにするために更新
- 更新したtermiosをもとにターミナルの状態を更新
となります。

// commitを貼る
挙動を確認してみましょう
canonicalモードと同じような操作をすると以下のような出力になりました

```
❯❯❯ cargo run
   Compiling toy-editor v0.1.0 (/home/ak-10/works/toy-editor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/toy-editor`
b: Ok(97)
         b: Ok(108)
                   b: Ok(102)
                             b: Ok(100)
```
canonicalモードと比べて出力が変ですが、文字単位の入力担っていることがわかります。
これでエディタ入力のベースができました。

フラグは32ビットで管理されていてそれぞれのビットを立てることで有効化します
例えばECHOを有効にしたい場合
```
// ECHO         -> '00000000000000000000000000001000'
// term.c_lflag -> '00000000000000000000000000000001' のとき
論理和を取ることで,もとのビットを保持しながら、有効にしたいビットのみ立てることができる

// term.c_lflag  = '00000000000000000000000000001001'
term.c_lflag |= ECHO;
```

逆に無効にしたい場合,
```
// !ECHO        -> '11111111111111111111111111110111'
// term.c_lflag -> '00000000000000000000000000000001' のとき
ビット反転したものとの論理積を取ることで,もとのビットを保持しながら、有効にしたいビットのみ落とすことができる

// term.c_lflag  = '00000000000000000000000000000001'
term.c_lflag &= !ECHO;
```
のようにすることで、特定のビットのみを立てることができます

ここで無効にしたフラグとtermiosの属性を書いておきます
rawモードではシグナルなどを送らないようにして文字入力のみにするイメージでいます(正直あまり理解できていません。)
- c_iflag:
  入力モードフラグ. 入力の諸々を設定する
   - IGNBRK:
    入力中のBREAK信号(ctrl + breakキー)を無視する.
    BREAK信号 = SIGQUIT?
  - BRKINT:
    IGNBRKが設定されている場合はBREAK信号を無視する.
    IGNBRKが設定されてないがBRKINTが設定されている場合はBREAK信号によって入出力キューがフラッシュされ、
    フォアグラウンドプロセスグループにSIGINTが送られる
    IGNBRK,BRKINTの両方が設定されていない場合, BREAKを'\0'(ヌル文字)として読み込む。
  - PARMRK:
    INPCK が設定され、IGNPAR が設定されていない場合にのみ効果がある
    IGNPARが設定されていない場合、パリティエラーまたはフレームエラーが発生した文字の前に'\377' '\0' を付与する
    IGNPAR, PARMRKの両方がセットされていない場合、パリティエラーまたはフレームエラーが発生した文字を'\0'として読み込む
  - ISTRIP:
    8ビット目を落とす
  - INLCR:
    入力のNL(new line: 改行文字)をCR(carriage return: 復帰文字)に置き換える
  - IGNCR:
    入力のCRを無視する
  - ICRNL:
    IGNCRが設定されていない場合、入力のCRをNLに置き換える
  - IXON:
    出力のXON/XOFFフロー制御を有効にする(よくわからない)

- c_oflag: 出力モードフラグ
  - OPOST:
    実装に依存した出力処理を有効にする(よくわからない)
- c_lflag: ローカルモードフラグ
  - ECHO:
    入力された文字をエコーする. エディタでの出力はターミナルの機能で出力をしないほうが都合が良い
  - ECHONL:
    ICANONが設定されていた場合、ECHOが設定されていなくてもNL文字をエコーする
  - ICANON:
    カノニカルモードを有効にする
    特殊文字EOF, EOL, EOL2, ERASE, KILL, LNEXT, REPRINT, STATUS, WERASE 行単位バッファが有効になる。(行単位入力)
  - ISIG:
    INTR, QUIT, SUSP, DSUSP の文字を受信した時,対応するシグナルを発生させる。
  - IEXTEN:
    実装依存の入力処理を有効にする
- c_cflag: 制御モードフラグ
  - CSIZE
    文字サイズを設定する
  - PARENB
    パリティビットを有効にする
  - CS8
    データビット数を8にする

また、rawモードではプログラムを終了させる方法がないため、control + qで終了させるようにします
control付きの文字(制御文字)は文字コードの0~31(0000_0000 ~ 0001_1111)に割り当てられています
control + qであれば17になります
これを判定するために `control_char` という関数を定義します
// commit を貼る


