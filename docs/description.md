## Rawモードの有効化
この章では入力について触れていきます
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
https://github.com/AK-10/toy-editor/pull/1/commits/817ceef2a06c63f927104591f68d03889d325c6e

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
    INTR(control + c), QUIT(control + \), SUSP, DSUSP の文字を受信した時,対応するシグナルを発生させる。
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
https://github.com/AK-10/toy-editor/pull/1/commits/6f62d565c256c0f63d4ba559c5f48cd4bc627906

## テキストを表示する
この章では引数で指定したファイルを表示するようにします

まずファイルパスを受け取り、ファイルの内容を取得部分を作成します
https://github.com/AK-10/toy-editor/pull/2/commits/ceec7118a0bf50e7a86ab874c8e2a0470b8db054

ファイルの内容は一旦Vec<String>として保持しておきます
これをそのままプリントすると以下のようになります
https://github.com/AK-10/toy-editor/pull/2/commits/1d0edda76e67199e3140a6ad29bd6a80c26c24a2

```
❯❯❯ cargo run -- examples/hello.txt
   Compiling toy-editor v0.1.0 (/home/ak-10/works/toy-editor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/toy-editor examples/hello.txt`
Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                                                        sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                                                                                                          Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
                                                                     Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
           Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

残念ながらrawモードではプリントを行ったあと、カーソルの行の位置がリセットされないため、次の列のプリントが前の行の文字数分スペースが入ってしまいます。
これを回避するためにカーソルを次の行の先頭に移動させる必要があります。
どうすればよいでしょうか？

ターミナルはエスケープシーケンスを使うことで制御できます。
例えば、カーソルの移動は
- 上: \x1b[A
- 下: \x1b[B
- 右: \x1b[C
- 左: \x1b[D
を入力することでカーソルを移動させることができます(\x1bはescを表します)
エスケープシーケンスについてはhttps://www.csie.ntu.edu.tw/~r92094/c++/VT100.htmlで確認できます
表を見てみると、\x1b[Eで次の行の先頭にカーソルを動かすことができそうです。
実際にやってみましょう
https://github.com/AK-10/toy-editor/pull/2/commits/93ea3783001fd82f8d7089b275232432477b16a0

```
~/w/toy-editor ❮ 22-12-08 0:49:50 ❯
❯❯❯ cargo run -- examples/hello.txt
   Compiling toy-editor v0.1.0 (/home/ak-10/works/toy-editor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/toy-editor examples/hello.txt`
Lorem ipsum dolor sit amet, consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

想定通りの出力になりました。

このように、ターミナルで動くエディタでは、エスケープシーケンスをうまく利用して、文字出力をしたり、キー入力とエスケープシーケンスを対応させることで動作を作っていきます

実際のエディタを考えてみると、
```
❯❯❯ cargo run -- examples/hello.txt
   Compiling toy-editor v0.1.0 (/home/ak-10/works/toy-editor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/toy-editor examples/hello.txt
```
のような、コマンドを実行したときの出力が残ると困ります。
また、ターミナルの先頭から出力するために、カーソルを左上に移動させる必要があります

画面のクリアには`\x1b[2J`, カーソルを左上に移動させるには`x1b[H`を使います
これをテキストを出力前に出力すれば良さそうです。
https://github.com/AK-10/toy-editor/pull/2/commits/bd4c2130920ab60b018d1bf0997a17e1fee2f8f4

あとは適当にリファクタしておきます
https://github.com/AK-10/toy-editor/pull/2/commits/8300c14817275155910696f4709fee5309d67360
https://github.com/AK-10/toy-editor/pull/2/commits/e29d080a71c6031d580bc5ac236e00ee87be85b6

## カーソルの移動
この章ではカーソル移動を実装していきます

まず、テキトーにカーソル移動に利用するキーを決めます
私はよくvimを使うので
- ctrl + h: 左
- ctrl + j: 下
- ctrl + k: 上
- ctrl + l: 右
のようにカーソル移動するようにします

まず各入力を受け入れるようにしましょう
入力を受け取る処理はすでに実装しているので、対応する分岐を書くだけで良いです
https://github.com/AK-10/toy-editor/pull/3/commits/1c81cca0f663dc209cb76afb4abf15cb9be549fe

入力を受け取ったらctrl + hjklに対応するエスケープシーケンスを出力する必要があります
カーソルを一つ移動させるエスケープシーケンスは
- 左: `\x1b[D`
- 下: `\x1b[B`
- 上: `\x1b[A`
- 右: `\x1b[C`

になります。
これをそのまま標準出力に吐けば良いです
https://github.com/AK-10/toy-editor/pull/3/commits/e4c454a430be56bdbab0bf915ef77c786779a13f

これでカーソル移動ができるようになりました。
エディタっぽくなってきましたね。

今の状態ではテキストを超えてカーソルが移動してしまいます。
テキストの範囲内でのみカーソルが移動できるようにしてみましょう。

- カーソルの位置を記憶しておく
- カーソルの位置とテキストの位置を対応させる
- カーソル移動入力を受け取ったときに、テキストの行と列の範囲を超えないかチェックし、超えない範囲であれば移動する
を実装すれば、良さそうです
https://github.com/AK-10/toy-editor/pull/3/commits/b32e027153f0e70268aae8ae44c0abe55768ce11

注意点としては、カーソルの上下移動をしようとしたとき、単純にカーソルを上下に移動させただけでは文字範囲からはみ出すことがあります

例として以下のようなテキストを考えます
```
short line.
loooooooooong line!
                  ^ ここにカーソルが存在する
```
この状態で単にカーソルを上に移動させようとすると

```
short line.
                  ^ ここにカーソルが移動する
loooooooooong line!
```
このようになります
これでは文字列の範囲を超えてしまうので、カーソルの列を移動後の文字列の末尾に移動させるようにします
また、これによってカーソルの移動に `\x1b[A`, `\x1b[B`, `\x1b[C`, `\x1b[D`を使うのは適切ではないので、他のエスケープシーケンスを使ったほうが良さそうです
`\x1b[v;hH` を利用するのが良さそうなので、こちらに変更します


## テキストへの書き込み
この章では書き込みができるようにしていきます

書き込みに必要な操作は
- キー入力を受け取る
- 受け取った文字を内部で持っているテキストに反映
- ターミナルに反映

になります

まず任意の文字を受け取れるようにしましょう

// commit を貼る
方針
- Editor構造体を作る
  - テキストへの書き込みはこの構造体の責務
  - 入力をどうするかは考える
    - terminalにもたせて良さそうな気がする
    - terminalもeditorに持たせる?
      - terminalでない入力機構の場合にどうするか
        - InputInterfaceを持ったものをeditorで持つ形であればよいか
  - rendererをeditorでもつか、別で持つか
    - どっちでもいい気はする
    - 一旦editorに持たせる
