# Specifikacija projekta  
## Veb aplikacija Gameup

---

## 1. Uvod  
Gameup je veb aplikacija namenjena pregledu video igara i gledanju njihovih trailer-a. Aplikacija korisnicima omogućava centralizovan i pregledan pristup informacijacijama o video igrama, uz multimedijalni sadržaj u vidu slika i video zapisa.

Projekat se realizuje kao projekat za 80 poena, sa ciljem primene mikroservisne arhitekture i savremenih tehnologija u razvoju veb aplikacija. Poseban akcenat stavljen je na backend sistem implementiran u programskom jeziku Rust, sa jasno razdvojenim odgovornostima kroz mikroservise.

---

## 2. Arhitektura sistema  
Sistem je baziran na mikroservisnoj arhitekturi i sastoji se iz sledećih celina:

- Frontend aplikacije implementirane u Angular-u  
- Backenda podeljenog na više mikroservisa  
- API Gateway servisa  
- PostgreSQL baze podataka  
- Docker kontejnera za svaki servis  

Svi backend servisi su razvijeni u programskom jeziku Rust i međusobno komuniciraju putem REST API-ja.

---

## 3. Funkcionalni zahtevi  

### 3.1. Funkcionalnosti za krajnje korisnike
- Prikaz liste video igara  
- Pretraga i filtriranje igara  
- Prikaz detalja o video igri  
- Prikaz slika povezanih sa igrom  
- Gledanje video trailer-a unutar aplikacije  

### 3.2. Administratorske funkcionalnosti
- Dodavanje, izmena i brisanje video igara  
- Upload i upravljanje slikama igara  
- Upload i upravljanje video fajlovima (trailer-ima)  

---

## 4. Backend sistem (Rust + mikroservisi)  

Backend deo sistema je u potpunosti implementiran u programskom jeziku Rust i organizovan kao skup nezavisnih mikroservisa, od kojih svaki ima jasno definisanu odgovornost.

### 4.1. API Gateway mikroservis  
API Gateway predstavlja centralnu ulaznu tačku za sve zahteve koji dolaze sa frontend aplikacije:

- Rukovanje HTTP zahtevima sa klijentske strane  
- Prosleđivanje zahteva odgovarajućim mikroservisima  
- Centralizovana validacija zahteva  
- Potencijalna autentifikacija i autorizacija  
- Omogućavanje lakše skalabilnosti i održavanja sistema  

### 4.2. Mikroservis za upravljanje igrama  
- Implementiran u Rust-u  
- Zadužen za CRUD operacije nad video igrama  
- Komunikacija sa PostgreSQL bazom podataka  
- Upravljanje metapodacima (naziv, opis, žanr, datum izlaska)  

### 4.3. Mikroservis za rad sa slikama  
- Implementiran u Rust-u  
- Omogućava upload slika (cover, screenshot-ovi)  
- Skladištenje i dohvat slika  
- Validacija formata i veličine slika  
- Isporuka slika frontend aplikaciji  

### 4.4. Mikroservis za rad sa video fajlovima  
- Implementiran u Rust-u  
- Omogućava upload video fajlova (trailer-a)  
- Skladištenje video sadržaja  
- Streaming video fajlova ka klijentu  
- Kontrola formata i veličine video fajlova  

---

## 5. Frontend sistem  
Frontend aplikacija se implementira korišćenjem Angular framework-a i obezbeđuje:

- Komunikaciju sa backend sistemom isključivo preko API Gateway-a
- Prikaz liste i detalja video igara  
- Prikaz slika i reprodukciju video trailer-a  

---

## 6. Baza podataka  
Za skladištenje podataka koristi se PostgreSQL baza podataka:

- Čuvanje podataka o video igrama
- Relacije između igara i multimedijalnog sadržaja

---

## 7. Kontejnerizacija sistema  
Kompletan sistem je kontejnerizovan korišćenjem Docker-a:

- Svaki mikroservis ima sopstveni Docker kontejner
- Poseban kontejner za PostgreSQL bazu podataka
- Definisanje Dockerfile i docker-compose.yml fajlova

---
