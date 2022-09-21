# Php tutorial

PHP Programming Language Tutorial - Full Course

[https://youtu.be/OK_JCtrrv-c](https://youtu.be/OK_JCtrrv-c)

<br>

<hr>

macOS install php

```
brew install php
```

설치하고 나서 라이브 서버 같은 서버 활성화 해주기

```
brew services restart php



php -S 127.0.0.1:8080
```

index.php 만들어 주고

chrome 열어서
http://localhost:8080

index.php 예제 코드

```
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title></title>
    </head>
    <body>

        <?php
            echo("Hello World");
        ?>
    </body>
</html>
```
