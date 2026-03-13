#set page(paper: "a4")
#set heading(numbering: "1.")
#set text(lang: "pl", size: 12pt)
#set par(first-line-indent: (amount: 1em, all: true))

#let date = datetime(
  year: 2025,
  month: 11,
  day: 30,
)

#[
  #set page(paper: "a4", margin: 2.5cm)
  #set align(center)

  #grid(
    columns: (1fr, 1fr, 2fr),
    align: (center + horizon, center + horizon),
    image("assets/LogoKIA.png", height: 100pt),
    grid.cell(""),
    image("assets/weii_pl.png", height: 100pt),
  )

  #v(3.5cm)
  #text(size: 26pt, weight: "bold")[Sprawozdanie\ Interakcja Człowiek-Komputer]
  #v(0.5cm)
  #text(size: 20pt)["Wykorzystanie wizji komputerowej\ do interakcji z komputerem"]

  #align(bottom)[
    #text(size: 14pt)[
      Damian Pietryka\
      3 EF-DI AA L03\
      177140
    ]
  ]

  #v(0.5cm)
  #text(size: 12pt)[Rzeszów\ #date.display("[day] listopad [year]")]

  #pagebreak()
]

= Temat ćwiczenia

W ramach ćwiczenia należało przygotować aplikację dla systemu MATLAB 7.1, której zadaniem była gra na wirtualnej klawiaturze wyświetlanej na tle obrazu otrzymanego z kamerki internetowej podłączonej do komputera. Wskazanie odpowiedniego klawisza przez użytkownika powodowało wygenerowanie dźwięku przypisanego do tego klawisza.

= Opis sposobu wykonania ćwiczenia

== Wariant 1 (odejmowanie tła):

Aplikacja wykonuje "zdjęcie" widoku z kamery (tła), które jest następnie porównywane z aktualnym obrazem z kamery. Jeżeli zostanie wykryty obiekt i znajdzie się nad klawiszem, zostanie zagrany wybrany dźwięk.

== Wariant 2 (Model koloru):

Aplikacja oblicza model koloru ze wskaźnika. Jeżeli zostanie wykryty obiekt w widoku kamery o podobnym modelu, zostanie on uznany za wskaźnik, jeżeli znajdzie się on nad klawiszem, zostanie zagrany wybrany dźwięk.

= Wartości parametrów

$s z e r \_ w s k$ = 2 --- Wartość 2 zmniejszała liczbę błędów względem wartości 1, ale była na tyle mała, że nawet cienki wskaźnik był wykrywany poprawnie. Utrzymanie małych wartości było ważne ze względu na potrzebę dużej dokładności w określaniu pozycji wskazywanej.

= Instrukcja dla użytkownika aplikacji

== Wariant 1 (odejmowanie tła):

Program musi być uruchamiany na statycznym tle (Brak ruchomych obiektów).

- Uruchom skrypt.
- Usuń się z kadru kamery i wciśnij _\<Enter\>_, aby program pobrał tło.
- Użyj dłoni lub dowolnego przedmiotu, aby naciskać klawisze.

== Wariant 2 (Model koloru):

Program musi być uruchamiany na tle, w którym nie występują obiekty o podobnym kolorze do wskaźnika.

- Uruchom skrypt.
- Umieść wskaźnik (np. kolorowy flamaster) w kadrze kamery i zaznacz go.
- Użyj tego samego przedmiotu, aby naciskać klawisze.

= Kod przygotowanej aplikacji

== Wariant 1 (odejmowanie tła):

#show figure.caption: it => {
  align(box(align(it, left)), center)
}

#figure(
  caption: "Program w wersji z odejmowaniem tła",
  box(width: 100%, align(
    ```matlab
    kamera_stop

    kyb = PrzygotujKlawiature();
    skala = WyliczSkale(vid, kyb);
    tlo = PobierzTlo(vid, skala);

    szer_wsk = 1;

    for i=1:200
        ob = PobierzBiezacyObraz(vid, skala);
        ob_klaw = NalozKlawiature(ob, kyb);
        figure(2);
        imshow (ob_klaw);
        dzwiek = ZnajdzKlawisz13tlo(ob, tlo, szer_wsk);
        if strcmp(dzwiek, '   ')==1
            Graj(dzwiek, 0.25)
        end
    end

    kamera_stop
    ```,
    left,
  )),
)

== Wariant 2 (Model koloru):

#figure(
  caption: "Program w wersji z modelem koloru",
  box(width: 100%, align(
    ```matlab
    kamera_stop

    kyb = PrzygotujKlawiature();
    skala = WyliczSkale(vid, kyb);
    probka_koloru = PobierzObrazModeluKoloru(vid);
    wzorzec_koloru = GenerujModel(probka_koloru);

    szer_wsk = 1;

    for i=1:200
        ob = PobierzBiezacyObraz(vid, skala);
        ob_klaw = NalozKlawiature(ob, kyb);
        figure(2);
        imshow (ob_klaw);
        dzwiek = ZnajdzKlawisz13model(ob, wzorzec_koloru, szer_wsk);
        if strcmp(dzwiek, '   ')~=1
            Graj(dzwiek, 0.25)
        end
    end

    kamera_stop;
    ```,
    left,
  )),
)


#pagebreak()

= Porównanie wersji aplikacji

#table(
  columns: 3,
  [*Warunki*], [*Wariant 1*], [*Wariant 2*],
  [Oświetlenie],
  [Jakakolwiek zmiana oświetlenia wymagała ponownego pobrania obrazu tła],
  [Niewielkie zmiany oświetlenia nie miały wpływu na działanie, ale większe zmuszały do ponownego pobrania modelu koloru wskaźnika],

  [Zmiana otoczenia / ruchy kamerą],
  [Przesunięcie kamery nawet o kilka milimetrów lub o niewielki nawet kąt zmuszało do ponownego pobrania obrazu tła],
  [Tak długo jak w kadrze nie znajdował się żaden obiekt o kolorze podobnym do wskaźnika aplikacja działała bez zarzutu],
)

= Wnioski

W przeprowadzonym ćwiczeniu zrealizowano dwa warianty aplikacji wykorzystującej techniki wizji komputerowej do interakcji z wirtualną klawiaturą. Oba podejścia pozwoliły na poprawne wykrywanie wskaźnika i generowanie dźwięków odpowiadających wskazywanym klawiszom.

Metoda odejmowania tła okazała się znacznie bardziej wrażliwa na warunki zewnętrzne. Nawet drobne zmiany oświetlenia lub minimalne przesunięcia kamery wymuszały ponowne pobranie obrazu referencyjnego. Oznacza to, że w praktycznych zastosowaniach wymaga ona bardzo stabilnego środowiska pracy.

Wariant oparty na modelu koloru wskaźnika wykazał się większą elastycznością i stabilnością działania. Niewielkie zmiany oświetlenia nie zaburzały poprawnego działania aplikacji, a sama interakcja była bardziej intuicyjna. Ograniczeniem tej metody jest potrzeba odpowiedniego dobrania koloru wskaźnika oraz brak obiektów w tle o tej samej barwie.

Podsumowując, obie metody pozwalają na skuteczną detekcję wskaźnika, jednak wariant z modelem koloru jest bardziej praktyczny i odporny na zakłócenia środowiskowe. Ćwiczenie pozwoliło również zrozumieć podstawowe problemy, jakie pojawiają się przy projektowaniu systemów interakcji opartych na obrazie — przede wszystkim wrażliwość na oświetlenie, kolorystykę sceny oraz stabilność położenia kamery.
