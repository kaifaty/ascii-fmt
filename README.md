# ascii-fmt

CLI утилита на Rust для пост-обработки ASCII/Unicode диаграмм: нормализует пробелы, выравнивает линии и конвертирует "ASCII-псевдографику" (`+ - | / \`) в Unicode box-drawing (`┌ ─ │ ┼ ╱ ╲`).

[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/ascii-fmt)](https://crates.io/crates/ascii-fmt)

## Быстрый старт

```bash
# Форматировать файл и сохранить результат
ascii-fmt diagram.txt -o diagram_fixed.txt

# Вывести в stdout
ascii-fmt diagram.txt

# Прочитать из stdin
cat diagram.txt | ascii-fmt > diagram_fixed.txt

# Только показать результат (ничего не писать в файл)
ascii-fmt diagram.txt --dry-run
```

## Установка

### Cargo (рекомендуется)

```bash
cargo install ascii-fmt
```

### Из исходников

```bash
git clone https://github.com/kaifaty/ascii-fmt.git
cd ascii-fmt
cargo install --path .
```

Опционально: `make install` (см. `make help`).

### npm (опционально)

В репозитории есть `package.json` (npm-обертка). При установке она собирает Rust бинарник локально.

```bash
# Внутри репозитория
npm i -g .
```

## Использование

Справка CLI: `ascii-fmt --help`

Команды:

- `ascii-fmt docs [topic]` - встроенная документация (`examples`, `api`, `integration`, `file-formats`)
- `ascii-fmt opencode-setup [--force]` - установить OpenCode plugin в `.opencode/plugins/ascii-fmt.js` в текущем проекте

Полезные опции:

- `-o, --output <FILE>` - записать в файл (по умолчанию stdout)
- `-s, --style <minimal|standard|detailed>` - стиль выравнивания текста
- `--dry-run` - выводит результат в stdout, не пишет файл
- `-v, --verbose...` - подробный вывод

### Примечания по текущей реализации

- `--style` сейчас влияет только на выравнивание текста: `detailed` центрирует строки; `minimal/standard` делают `trim()` + добивка пробелами до исходной ширины.
- `--width` работает как ручной override для ширины сетки (если поставить `> 2`), иначе ширина определяется автоматически по диаграмме.
- `--preserve-empty-lines=false` удаляет пустые/whitespace-only строки из вывода.
- `--fix-box-drawing` и `--fix-whitespace` в CLI включены по умолчанию; отдельного режима "выключить" пока нет.
- Если прогонять по обычным Markdown-таблицам, утилита может конвертировать `|` в `│`; для `.md` безопаснее использовать OpenCode plugin (он трогает только отмеченные fenced-блоки).

## OpenCode: автоформатирование файлов

`ascii-fmt` умеет сгенерировать project-level plugin для OpenCode:

```bash
ascii-fmt opencode-setup

# Если нужно перезаписать существующий файл
ascii-fmt opencode-setup --force
```

Результат: `.opencode/plugins/ascii-fmt.js`

Поведение плагина (см. `scripts/opencode-plugin.js`):

- Markdown: форматирует только fenced-блоки с языками `ascii`, `diagram`, `ascii-diagram`
- Остальные файлы: форматирует целиком только если содержимое "похоже на диаграмму" (консервативная эвристика)
- Ошибки: fail-open - пишет warning в лог OpenCode и не блокирует работу

Если вы ставите пакет через npm и он опубликован, доступен хелпер:

```bash
npx ascii-fmt-opencode-setup
```

(он создаёт `.opencode/plugins/ascii-fmt.js`, но не перезаписывает существующий файл).

### Как помечать диаграммы в Markdown

````markdown
```ascii
┌───┐
│ A │
└───┘
```
````

## Примеры

### Architecture

Input:

```
┌─────────────────────────┐
│   API Gateway          │
├───────────┬────────────┤
│    User   │   Service  │
│  Service  │  Layer     │
└───────────┴────────────┘
     |
     v
   Database
```

Output:

```
┌─────────────────────────┐
│      API Gateway        │
├─────────────┬───────────┤
│  User       │  Service  │
│  Service    │  Layer    │
└─────────────┴───────────┘
      │
      │
      ▼
   Database
```

### Flowchart

Input:

```
+-------+       +-------+
|  A    |------>|  B    |
+-------+       +-------+
     |
     v
  +-----+
  |  C  |
  +-----+
```

Output:

```
┌───────┐       ┌───────┐
│  A    │──────▶│  B    │
└───────┘       └───────┘
    │
    │
    ▼
  ┌─────┐
  │  C  │
  └─────┘
```

## Разработка

```bash
cargo test
cargo fmt
cargo clippy -- -D warnings
```

См. `Makefile` (цели `build`, `release`, `test`, `fmt`, `clippy`, `cross-*`, `package-*`).

## License

MIT, см. `LICENSE`.

## Ссылки

- https://crates.io/crates/ascii-fmt
- https://docs.rs/ascii-fmt
- https://github.com/kaifaty/ascii-fmt
- https://github.com/kaifaty/ascii-fmt/issues
