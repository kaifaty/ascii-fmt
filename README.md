# ASCII Diagram Formatter

CLI инструмент на Rust для автоматического исправления и выравнивания ASCII диаграмм, созданных AI агентами.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/ascii-fmt)](https://crates.io/crates/ascii-fmt)

## Описание

`ascii-fmt` автоматически исправляет типичные ошибки, которые делают AI агенты при генерации ASCII диаграмм:

- ❌ **До**: Смещенные строки, некорректные box-drawing символы, inconsistent spacing
- ✅ **После**: Выровненные по сетке диаграммы с правильными Unicode символами

## Возможности

### Основные фичи

- ✅ **Автовыравнивание по сетке** - Определяет характерную ширину ячеек и выравнивает строки
- ✅ **Исправление box-drawing символов** - Заменяет `+ - | / \` на правильные `┼ ─ │ ╱ ╲`
- ✅ **Умное выравнивание текста** - Центрирование заголовков, выравнивание содержимого блоков
- ✅ **Нормализация пробелов** - Удаляет лишние пробелы, добавляет недостающие для сетки
- ✅ **Паттерн-матчинг** - Flowchart, Tree, Architecture, Sequence diagrams

### Поддерживаемые типы диаграмм

| Тип | Пример |
|-----|--------|
| Flowchart | Стрелки, блоки решений |
| Tree diagrams | Иерархические ветвления |
| Architecture | Слоистые архитектуры, компоненты |
| Sequence diagrams | Вертикальные линии, сообщения |
| ASCII tables | Таблицы с границами |

## Установка

### Из crates.io

```bash
cargo install ascii-fmt
```

### Из исходников

```bash
git clone https://github.com/kaifaty/ascii-fmt.git
cd ascii-fmt
cargo install --path .
```

### Скачать готовые бинарники

Готовые бинарники для разных платформ доступны в [GitHub Releases](https://github.com/kaifaty/ascii-fmt/releases).

Поддерживаемые платформы:
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Linux**: x86_64 (glibc), x86_64 (musl, static binary), aarch64 (ARM64), armv7 (Raspberry Pi)
- **Windows**: x86_64

## Кросс-компиляция

### Локальная сборка с Makefile

Для удобства добавлен Makefile с целями для кросс-компиляции:

```bash
# Показать все доступные команды
make help

# Сборка для текущей платформы
make build
make release

# Сборка для других платформ (требует cross)
make install-cross
make cross-all              # Сборка всех поддерживаемых платформ
make package-all            # Упаковка всех бинарников с checksums

# Специфичные цели
make cross-x86_64-apple-darwin       # Intel macOS
make cross-aarch64-apple-darwin       # Apple Silicon
make cross-x86_64-unknown-linux-gnu   # Linux x86_64
make cross-aarch64-unknown-linux-gnu # Linux ARM64
make cross-x86_64-pc-windows-msvc    # Windows x86_64
```

### Использование cross tool

Для кросс-компиляции Linux/Windows с macOS используется инструмент `cross`:

```bash
# Установка cross
cargo install cross --git https://github.com/cross-rs/cross

# Примеры использования
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-msvc
```

### Cargo aliases

В `.cargo/config.toml` добавлены удобные алиасы:

```bash
cargo x86-apple      # x86_64-apple-darwin
cargo arm-apple      # aarch64-apple-darwin
cargo x86-linux      # x86_64-unknown-linux-gnu
cargo arm-linux      # aarch64-unknown-linux-gnu
cargo x86-windows    # x86_64-pc-windows-msvc
```

## Использование

### Базовое использование

```bash
# Чтение из файла, вывод в stdout
ascii-fmt diagram.txt

# Запись в файл
ascii-fmt diagram.txt > output.txt

# Чтение из stdin
cat diagram.txt | ascii-fmt > output.txt
```

## Интеграция с OpenCode (автоформатирование после записи файлов)

OpenCode умеет загружать project-level плагины из `.opencode/plugins/` и вызывать их на событиях файлов (например, `file.edited`). Это позволяет автоматически запускать `ascii-fmt` каждый раз, когда агент записал файл с диаграммой.

Официальная документация OpenCode:
- https://opencode.ai/docs/plugins/
- https://opencode.ai/docs/config/
- https://opencode.ai/docs/formatters/

### Быстрая настройка (рекомендуется)

В корне вашего проекта:

```bash
# (1) Убедитесь, что ascii-fmt установлен и доступен в PATH
cargo install ascii-fmt

# (2) Сгенерируйте OpenCode plugin в .opencode/plugins/
ascii-fmt opencode-setup
```

Если вы ставите `ascii-fmt` через npm, можно запустить установщик напрямую (без cargo):

```bash
npx ascii-fmt-opencode-setup
```

В результате появится файл:
- `.opencode/plugins/ascii-fmt.js`

### Как помечать диаграммы в Markdown

Плагин форматирует только fenced-блоки с языками `ascii`, `diagram`, `ascii-diagram`.

```markdown
```ascii
+---+
| A |
+---+
```
```

### Поведение и ограничения

- Markdown: форматируются только помеченные fenced-блоки; остальной текст файла не трогается.
- Не-Markdown: файл форматируется целиком только если он похож на диаграмму (консервативная эвристика), чтобы не ломать обычный текст.
- Ошибки форматирования: fail-open (плагин логирует warning через OpenCode и не блокирует работу).

### Опции

```bash
ascii-fmt [INPUT] [OPTIONS]

Arguments:
  [INPUT]    Input file or stdin (default)

Options:
  -o, --output <FILE>       Output file or stdout
  -w, --width <N>           Target grid cell width [default: 2]
  -s, --style <STYLE>       Style preset: minimal, standard, detailed [default: standard]
      --fix-box-drawing     Enable box-drawing symbol fixes [default: true]
      --fix-whitespace      Normalize whitespace [default: true]
      --preserve-empty-lines  Don't remove empty lines [default: true]
      --dry-run             Show changes without writing
  -v, --verbose             More detailed output
  -h, --help                Print help
  -V, --version             Print version
```

## Примеры

### Пример 1: Architecture Diagram

**Input** (кривой):
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

**Output** (исправленный):
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

```bash
ascii-fmt arch_diagram.txt
```

### Пример 2: Flowchart

```bash
ascii-fmt --style detailed flowchart.txt > fixed_flowchart.txt
```

### Пример 3: Tree Diagram

```bash
ascii-fmt --width 4 tree.txt
```

### Пример 4: Dry-run (проверка изменений)

```bash
ascii-fmt --dry-run diagram.txt
```

## Стили

### Minimal
Только критические исправления, минимальные изменения:
- Базовое выравнивание по сетке
- Исправление очевидно неправильных символов

### Standard (по умолчанию)
Сбалансированный набор исправлений:
- Выравнивание по сетке
- Исправление box-drawing символов
- Нормализация пробелов
- Базовое выравнивание текста

### Detailed
Максимум исправлений:
- Все исправления standard
- Центрирование заголовков
- Сохранение отступов для вложенности
- Расширенная валидация соединений

## Архитектура

```
src/
├── main.rs           # CLI entry point
├── cli.rs            # Argument parsing with clap
├── formatter.rs      # Main formatting logic (orchestrator)
├── parser.rs         # ASCII diagram parsing
├── grid.rs           # Grid alignment logic
├── box_drawing.rs    # Box-drawing symbol fixes
├── patterns.rs       # Pattern matching for diagram types
├── text_align.rs     # Text alignment utilities
└── utils.rs          # Helper functions

tests/
├── fixtures/         # Sample diagrams for testing
│   ├── flowcharts/
│   ├── architecture/
│   ├── trees/
│   ├── sequence/
│   └── tables/
├── integration_test.rs
└── unit_tests.rs

benches/
└── formatter_benchmark.rs
```

## Алгоритм работы

1. **Parse Phase**
   - Разбиваем ввод на строки
   - Определяем тип диаграммы по паттернам
   - Создаем абстрактное представление (AST)

2. **Grid Analysis**
   - Определяем характерную ширину колонок
   - Находим точки выравнивания
   - Вычисляем сетку

3. **Apply Fixes**
   - Исправляем box-drawing символы
   - Нормализуем пробелы
   - Выравниваем текстовое содержимое

4. **Render**
   - Генерируем исправленный ASCII
   - Применяем сетку
   - Добавляем нормализованные пробелы

## Разработка

### Запуск тестов

```bash
# Все тесты
cargo test

# Тесты с выводом
cargo test -- --nocapture

# Benchmarks
cargo bench
```

### Linting

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

### Documentation

```bash
# Генерация документации
cargo doc --open

# Включая приватные элементы
cargo doc --document-private-items --open
```

## Требования

### Технические

- Rust 1.70+
- Cargo

### Performance Goals

- <10ms для диаграмм до 1000x1000 символов
- O(n) по размеру входа
- Memory: O(n) минимальный overhead

### Quality Goals

- >80% test coverage
- Zero `unsafe` code
- Zero clippy warnings
- Full API documentation

## Roadmap

### v1.0 (текущая)
- [x] Базовое выравнивание по сетке
- [x] Исправление box-drawing символов
- [x] Нормализация пробелов
- [x] Паттерн-матчинг для основных типов диаграмм
- [ ] CLI интерфейс
- [ ] ТестоваяCoverage >80%

### v2.0
- [ ] Плагины для разных языков разметки (Mermaid, PlantUML → ASCII)
- [ ] Цветовая подсветка
- [ ] Экспорт в другие форматы (SVG, PNG)
- [ ] Расширенные пресеты стилей

## CI/CD

Проект использует GitHub Actions для автоматической сборки бинарников под все поддерживаемые платформы.

### Release Workflow

При пуше тега (например, `v1.0.0`) автоматически:

1. Строятся бинарники для всех платформ
2. Создаются архивы (tar.gz для Unix, zip для Windows)
3. Генерируются SHA256 checksums
4. Создается GitHub Release с артефактами

### Поддерживаемые в CI платформы

```yaml
macOS:
  - x86_64-apple-darwin (Intel)
  - aarch64-apple-darwin (Apple Silicon)

Linux:
  - x86_64-unknown-linux-gnu (glibc)
  - x86_64-unknown-linux-musl (static binary)
  - aarch64-unknown-linux-gnu (ARM64)
  - armv7-unknown-linux-gnueabihf (ARMv7)

Windows:
  - x86_64-pc-windows-msvc
```

### Локальный тестинг CI

```bash
# Запуск всех тестов (как в CI)
make test
make clippy
make fmt-check
```

## Contributing

Contributions приветствуются! Пожалуйста, ознакомьтесь с [CONTRIBUTING.md](CONTRIBUTING.md) перед началом работы.

### Как начать

```bash
# Fork репозитория
# Клонируйте ваш форк
git clone https://github.com/kaifaty/ascii-fmt.git
cd ascii-fmt

# Создайте ветку для фичи
git checkout -b feature/amazing-feature

# Внесите изменения и протестируйте
cargo test

# Коммит и push
git commit -m "Add amazing feature"
git push origin feature/amazing-feature

# Создайте Pull Request
```

## Troubleshooting

### Проблема: Диаграмма не фиксируется корректно

**Решение**: Попробуйте разные стили:
```bash
ascii-fmt --style minimal diagram.txt
ascii-fmt --style detailed diagram.txt
```

### Проблема: Проблемы с кодировкой

**Решение**: Убедитесь, что ваш терминал поддерживает UTF-8:
```bash
locale | grep UTF-8
```

### Проблема: Медленная работа на больших файлах

**Решение**: Используйте release build:
```bash
cargo install --release ascii-fmt
```

## License

Этот проект распространяется под лицензией MIT. См. файл [LICENSE](LICENSE) для деталей.

## Acknowledgments

- Идея вдохновлена [ascii-guard](https://github.com/fxstein/ascii-guard)
- Box-drawing символы из Unicode Block Elements (U+2500-U+257F)
- Алгоритмы выравнивания основаны на лучших практиках из [boxes](https://boxes.thomasjensen.com/config-best-practices.html)

## Ссылки

- [Crates.io](https://crates.io/crates/ascii-fmt)
- [Documentation](https://docs.rs/ascii-fmt)
- [GitHub Issues](https://github.com/kaifaty/ascii-fmt/issues)
- [Changelog](CHANGELOG.md)

---

Made with ❤️ by the Rust community
