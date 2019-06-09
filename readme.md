[https://travis-ci.org/riros/rust_test_01.svg?branch=master](https://travis-ci.org/riros/rust_test_01.svg?branch=master)

Task:
-------

**Реализовать простое REST API с одним единственным методом, который загружает изображения.**


Требования:
- [X] Возможность загружать несколько файлов.
- [X] Возможность принимать multipart/form-data запросы.
- [X] Возможность принимать JSON запросы с BASE64 закодированными изображениями.
- [X] Возможность загружать изображения по заданному URL (изображение размещено где-то в интернете).
- [X] Создание квадратного превью изображения размером 100px на 100px.
- [X] Наличие модульных/интеграционных тестов.

Временем и инструментом для выполнение тестового задания Вы не ограничены.
 Любые другие аспекты реализации, которые не указаны в требованиях, могут быть выполнены на Ваше усмотрение.

Следующее будет плюсом:
- [X] \(Optional) Корректное завершение приложения при получении сигнала ОС (graceful shutdown).
- [X] \(Optional) Dockerfile и docker-compose.yml, которые позволяют поднять приложение единой docker-compose up командой.
- [ ] \(Optional) CI интеграция (Travis CI, Circle CI, другие).

Тестовое задание должно быть предоставлено в виде ссылки на публичный репозиторий (GitHub, BitBucket, GitLab),
 содержащий исходный код приложения и необходимые инструкции по сборке и запуску.


TR Logic LLC <contact@itprojects.management>

___
## Решение:

___
### Install

##### Clone Git
```
    $   git clone https://github.com/riros/rust_test_01.git
    $   cd rust_test_01
```
##### Install Rust
```
    $   curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
```
##### Run 
```
    $   cagro test
    $   cargo run
```

---
### Use Docker
```
    $   docker-compose up
```

----
### Use
- [http://localhost:8000/imgtest/v1](http://localhsot:8000/imgtest/v1).  
    applications/json , multipart/form-data requests supported.  
- run tests
    ```
    $   cargo test
    ```
- see tests code
___
### Tips
- *Библиотека ruster очень медленная и не удобная, не стал время тратить на эксперементы и поиски быстрой.*
    Конечно можно было использовать встроенную, но, как повезло.  
- *Время на рефакторинг не тратил*
- *Unit тестов нет, только интеграционные*
- *Миниатюры изображений сохраняются в папку media/thumbnails*

---
### Todo
- More error handling
- refactoring
  - migrate to tide
  - watermark functional
  - replace raster
