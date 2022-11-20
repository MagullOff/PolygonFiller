# PolygonFiller
## Instrukcja działania
### Uruchomienie aplikacji
Aby uruchomić aplikację należy w głównym katalogu projektu wykonać komędę ```cargo run --release```. Ponieważ aplikacja jest dość złożona obliczeniowo uruchamianie jej w trybie debug znacznie ją spowalnia i nie jest zalecane. Należy wspomnieć o aktualizacji lub pobraniu języka rust z <a href="https://www.rust-lang.org/tools/install">oficjalnych źródeł</a>.
### Obsługa
W lewym górnym rogu aplikacji znajduje się zakładka "Settings" która odpowiada za obsługę programu. 

Na górze menu znajduje się Checkbox odpowiadający za uruchomianie rotacji światła oraz ładowanie własnego modelu kształtu.

Suwaki w dziale "Coefficients" odpowiadają za kolejne parametry przedstawione w specyfikacji projektu

W dziale "Interpolation" można wybrać czy kolory mają być interpolowane z wektorów normalnych czy też kolorów na wierzchołkach poligonów.

W dziale "Colors and textures" można wybrać kolor światła, to czy kolor obiektu będzie ładowany z koloru wskazanego w menu czy z tekstury, którą można w tym miejscu również załadować, oraz czy mapa wektorów normalnych obiektu ma być modyfikowana o dodatkowo załadowaną mapę, którą można wybrać za pomocą znajdującego się obok przycisku.
