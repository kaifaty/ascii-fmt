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
git clone https://github.com/yourusername/ascii-fmt.git
cd ascii-fmt
cargo install --path .
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
│      API Gateway       │
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

## Contributing

Contributions приветствуются! Пожалуйста, ознакомьтесь с [CONTRIBUTING.md](CONTRIBUTING.md) перед началом работы.

### Как начать

```bash
# Fork репозитория
# Клонируйте ваш форк
git clone https://github.com/yourusername/ascii-fmt.git
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
- [GitHub Issues](https://github.com/yourusername/ascii-fmt/issues)
- [Changelog](CHANGELOG.md)

---

Made with ❤️ by the Rust community
