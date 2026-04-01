# 🛠️ Cad-Cs: Workflow & Cheatsheet

Ten plik zawiera osobiste notatki, triki i skróty przyspieszające pracę nad projektem w środowisku VS Code.

---

## 🔍 VS Code: Precyzyjne Wyszukiwanie i Zamiana (Search & Replace)

Kiedy robisz globalny refaktor (np. zmianę nazw struktur DTO lub metod), użyj poniższego filtru, aby uniknąć przypadkowej modyfikacji logów, plików wygenerowanych w `.history/` czy folderu `target/`.

Wklej poniższą linijkę w pole **"files to include"** (pliki do uwzględnienia) w panelu globalnego wyszukiwania (`Ctrl + Shift + F`):

```text
./src/**/*.rs, ./examples/**/*.rs, ./ARCHITECTURE.md, ./TODO.md, ./ROADMAP.md
