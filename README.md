# KSC (KILLER SUDOKU COMBO)

Простой помошник в решении Killer Sudoku(конечно на самом сложном уровне 😉)  
Это один из вариантов моей войны с дименцией... или альцгеймером... или просто с плохой памятью? Наверно все-таки за свободу. Но чего?)  
Веб-версию со своего сайта https://jager.site/ksc на Leptos с помощью Tauri портировал в приложение. Это было очень легко, респект и уважение Tauri.

<p align="center">
  <img src="screenshots/android.png" width="20%" alt="Android screenshot 0">
  <img src="screenshots/android1.png" width="20%" alt="Android screenshot 1">
  <img src="screenshots/ubuntu.png" width="20%" alt="Ubuntu screenshot 0">
  <img src="screenshots/ubuntu.png" width="20%" alt="Ubuntu screenshot 1">
</p>

## Особенности
- ✓ Минимализм как в приложении, так и в размерах релизов(~4 мб на). Есть темная и светлая темы. Нет меню, только выпадающий список About. Даже Help нет - кто увлекается killer sudoku сразу поймет.
- ✓ Расчет результатов основан на **binary search**, а не просто выдача из списка готовых решений.
- ✓ Кроссплатформенность. В релизах есть установщики для linux, android. Tauri позволяет сделать их и для windows, ios, но это уже не **таков путь**.

## Технологии
- [Tauri](https://github.com/tauri-apps/tauri) - сборка приложений на Rust
- [Leptos](https://github.com/leptos-rs/leptos) - веб-фреймворк на Rust

# Install
You can find the android, rpm, deb or appimage in the Releases page.
### Android
- Скачайте APK из релизов
- Разрешите установку из неизвестных источников
- Установите вручную
### Linux (.deb)
```bash
wget https://github.com/jager1win/ksc/releases/download/v1.0.0/your_app.deb
sudo dpkg -i your_app.deb
```
### Linux (.rpm)
```bash
wget https://github.com/jager1win/ksc/releases/download/v1.0.0/your_app.rpm
sudo dpkg -i your_app.rpm
```
## Check sign
```bash
gpg2 --verify package.deb.sig package.deb
```
## Статус проекта
Проект завершен и не будет развиваться дальше, так как:
- Выполнил свою задачу: позволил изучить и попрактиковаться в Tauri
- Работает стабильно в текущей версии, не требует доработок для моих нужд

## Лицензия
MIT - [LICENSE](LICENSE)

## P.S.
Самое сложное было настроить параметры релизов в tauri.conf.json. Из коробки плохо работало с linux и не работало с android.
Помогло создание отдельных конфигов для разных платформ.



