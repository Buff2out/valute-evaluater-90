# Подсчёт валют за 90

## Установка Rust

Следуем инструкции(https://www.rust-lang.org/tools/install).  
Инструкции примерно такие:  

### For macOS, Linux, or another Unix-like OS:  
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### For Windows  
Установить скачав и запустив rustup-init.exe:  
https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

Далее следуем инструкции в терминале, для windows (в целях обратной совместимости с C и C++) установщик может поставить Visual Studio только нужные компоненты. 


## Клонируем этот репозиторий либо копипастим скрипт в main.rs


### Если клонируем репозиторий:

`git clone https://github.com/Buff2out/valute-evaluater-90.git`

`cd valute-evaluater-90`

`cargo run`

>Rust компилируемый язык программирования, поэтому нужно немного подождать, пока он скомпилирует все зависимости и запустит исполняемый бинарник😎

Хотя скорость запросов имеет свои ограничения, запуск в режиме --release кардинально не улучшит ситуацию, поскольку львиную долю времени съедают именно запросы. Но по желанию можно:  
`cargo run --release`

при повторном запуске Debug/Release версии, бинарники хранятся в кэше и пересборка не понадобится до первых изменений в скрипте (но и там он не будет повторно долго билдить библиотеки, которые остались без изменений, а только сам `main.rs` ).

### Если копипастим скрипт:

#### Создаём проект

>cargo new создаст нам проект с инициализированным локально git-репозиторием и важным конфигурационным файлом зависимостей `Cargo.toml`

`cargo new blablabla`

`cd blablabla`

#### Копируем зависимости в Cargo.toml

При инициализации конфиг будет выглядеть примерно так:

Cargo.toml:  
```
[package]
name = "blablabla"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Нам остаётся просто добавить после `[dependencies]` вот это:  

```
chrono = "0.4"
quick-xml = { version = "0.28", features = ["serialize"] }
reqwest = { version = "0.11", features = ["blocking"] }
serde = { version = "1.0", features = ["derive"] }
```

>Итого, наш Cargo.toml будет таким


Cargo.toml:  
```
[package]
name = "blablabla"
version = "0.1.0"
edition = "2024"

[dependencies]
chrono = "0.4"
quick-xml = { version = "0.28", features = ["serialize"] }
reqwest = { version = "0.11", features = ["blocking"] }
serde = { version = "1.0", features = ["derive"] }
```

Можем не беспокоиться, зависимости сами к нам подгрузятся, никаких действий с ними дополнительно делать не нужно

#### Копируем код

Далее копируем из  
https://github.com/Buff2out/valute-evaluater-90/src/main.rs  
в  
`./src/main.rs` 

#### Запускаем программу:

`cargo run`