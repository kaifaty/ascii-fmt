# Contributing to ascii-fmt

Спасибо за интерес к внесению вклада в `ascii-fmt`! 🎉

Этот документ поможет вам начать работу над проектом.

## Как внести вклад

### Сообщение об ошибках (Bug Reports)

Если вы нашли баг, пожалуйста, создайте issue со следующей информацией:

1. **Описание**: Четкое и краткое описание проблемы
2. **Воспроизведение**:
   - Входные данные (ASCII диаграмма)
   - Ожидаемый результат
   - Фактический результат
3. **Окружение**:
   - Версия `ascii-fmt`: `ascii-fmt --version`
   - ОС: (Linux/macOS/Windows)
   - Версия Rust: `rustc --version`

### Запрос функций (Feature Requests)

Если у вас есть идея для улучшения, создайте issue и опишите:

1. **Зачем**: Какую проблему решает эта фича?
2. **Как**: Как вы видите реализацию?
3. **Примеры**: Конкретные примеры использования

### Pull Requests

PR приветствуются! Перед созданием PR:

1. **Форкните репозиторий** и создайте ветку для вашей фичи
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Сделайте изменения** и убедитесь, что тесты проходят
   ```bash
   cargo test
   ```

3. **Запустите linting** и исправьте предупреждения
   ```bash
   cargo clippy -- -D warnings
   cargo fmt
   ```

4. **Коммитните** с понятным сообщением
   ```bash
   git commit -m "Fix: Align box borders correctly"
   ```

5. **Пушните** и создайте PR
   ```bash
   git push origin feature/amazing-feature
   ```

## Стандарты кода

### Форматирование

Используйте `rustfmt`:
```bash
cargo fmt
```

### Linting

Используйте `clippy`:
```bash
cargo clippy -- -D warnings
```

**Обязательные правила**:
- ❌ Не использовать `unwrap()` - используйте `?` для propagation ошибок
- ❌ Не использовать `expect()` в production code
- ❌ Не подавлять ошибки (`catch e {}`)
- ✅ Использовать `crate::error::Result` + `thiserror` для error handling
- ✅ Добавлять unit tests для новой функциональности

### Документация

- Добавляйте doc comments для публичных API:
  ```rust
  /// Parses ASCII diagram into AST.
  ///
  /// # Arguments
  ///
  /// * `input` - Raw ASCII diagram as string
  ///
  /// # Errors
  ///
  /// Returns `ParseError` if input is invalid
  pub fn parse(input: &str) -> Result<ParsedDiagram>
  ```

- Обновляйте `README.md` и `ARCHITECTURE.md` при изменениях API

## Структура проекта

```
src/
├── main.rs           # CLI entry point
├── cli.rs            # Argument parsing
├── formatter.rs      # Main formatting logic
├── parser.rs         # ASCII diagram parsing
├── grid.rs           # Grid alignment
├── box_drawing.rs    # Box-drawing fixes
├── patterns.rs       # Pattern matching
├── text_align.rs     # Text alignment
└── utils.rs          # Helpers
```

При добавлении новой функциональности:

1. Создайте отдельный модуль в `src/`
2. Добавьте модуль в `src/main.rs` или соответствующий родительский модуль
3. Напишите unit tests в том же файле (внутри `#[cfg(test)]` модуля)
4. Добавьте integration tests в `tests/`
5. Обновите документацию

## Тестирование

### Unit Tests

Внутри каждого модуля:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_box_char() {
        // Arrange
        let input = '+';
        let context = /* ... */;

        // Act
        let result = fix_char(input, context).unwrap();

        // Assert
        assert_eq!(result, '┼');
    }
}
```

### Integration Tests

В `tests/integration_test.rs`:
```rust
#[test]
fn test_format_architecture_diagram() {
    let input = include_str!("fixtures/architecture/bad.txt");
    let expected = include_str!("fixtures/architecture/good.txt");

    let output = ascii_fmt::format_ascii(input, &Options::default()).unwrap();
    assert_eq!(output, expected);
}
```

### Добавление Fixtures

Для новых типов диаграмм:
1. Создайте файл в `tests/fixtures/<diagram_type>/bad.txt`
2. Создайте соответствующий файл `good.txt` с ожидаемым результатом
3. Добавьте integration test

## Benchmarks

При добавлении критических для производительности изменений:

1. Добавьте benchmark в `benches/formatter_benchmark.rs`:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn bench_new_feature(c: &mut Criterion) {
       let input = /* ... */;
       c.bench_function("new_feature", |b| {
           b.iter(|| new_feature(black_box(input)))
       });
   }

   criterion_group!(benches, bench_new_feature);
   criterion_main!(benches);
   ```

2. Запустите benchmarks:
   ```bash
   cargo bench
   ```

3. Убедитесь, что производительность не ухудшилась

## Process для Merge

### Small Changes

- Добавь unit tests → Запусти тесты → Commit → PR

### Medium Changes

- Добавь unit tests → Запусти тесты
- Добавь/обнови fixtures → Запусти integration tests
- Обнови README/ARCHITECTURE.md если нужно → Commit → PR

### Large Changes

- Откройте issue для обсуждения дизайна
- Получите апрув от maintainer
- Реализуйте поэтапно с тестами
- Код ревью → PR

## Review Process

После создания PR:

1. **Автоматические проверки**:
   - ✅ CI passes (тесты, clippy, fmt)
   - ✅ Coverage > 80%

2. **Код ревью**:
   - Хотя бы один maintainer одобряет
   - Все замечания должны быть решены

3. **Merge**:
   - Squash and merge
   - Сообщение следует Conventional Commits:
     - `fix: correct box border alignment`
     - `feat: add tree diagram support`
     - `docs: update README`

## Задачи для начинающих

Хорошие задачи для первого вклада:

1. **Добавить новые fixtures**
   - Создать примеры кривых диаграмм для edge cases

2. **Улучшить documentation**
   - Добавить примеры в README
   - Улучшить комментарии в коде

3. **Unit tests**
   - Увеличить coverage до >80%
   - Добавить tests для edge cases

Ищите issues с тегом `good first issue`!

## Связь

- **GitHub Issues**: Для багов и фич
- **Discussions**: Для вопросов и обсуждений
- **Email**: [your email]

## Code of Conduct

Будьте уважительны к другим участникам. Важно:
- Быть конструктивным в отзывах
- Принимать критику спокойно
- Сосредоточиться на том, что лучше для сообщества

Нарушения могут привести к запрету на участие.

## License

Внося свой вклад, вы соглашаетесь на то, что ваш вклад будет лицензирован под MIT License.

---

Спасибо за вклад! ❤️
