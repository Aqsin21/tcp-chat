# TCP Chat

Tokio tabanlı, çok istemcili ve satır tabanlı bir TCP broadcast chat sunucusu.
Sunucu `127.0.0.1:8080` üzerinde çalışır; bir istemciden gelen her mesaj,
gönderen dahil bağlı istemcilere iletilir.

## Amaç

Proje, Rust ile TCP ağ programlamasını küçük ve anlaşılır adımlarla öğrenmek
amacıyla geliştirilmiştir. Uygulama önce senkron echo sunucusu olarak başlamış,
ardından asenkron bağlantı yönetimi ve çok istemcili mesaj yayınlama aşamalarına
ilerlemiştir.

Geliştirme süreci Spec-Driven Development yaklaşımını izler. Her milestone
önce `spec`, `architecture`, `tasks` ve `decisions` belgeleriyle planlanır;
uygulama küçük, açıklanabilir ve doğrulanabilir adımlarla ilerletilir.

## Tamamlanan Milestone'lar

- `std::net` kullanan senkron TCP echo server
- Tokio tabanlı asenkron echo server
- Her bağlantı için ayrı task kullanan çok istemcili broadcast chat
- İstemci bağlantısı kapandığında ilgili task ve kaynakların temizlenmesi
- Gönderici dahil tüm bağlı istemcilere mesaj iletimi
- Otomatik Tokio testleri
- Birden fazla `nc` istemcisiyle manuel doğrulama talimatları

## Teknoloji ve Bağımlılıklar

- Rust 2024 edition
- Tokio `1.54.1`
- `tokio::net::TcpListener`
- `tokio::sync::broadcast`
- Satır tabanlı TCP protokolü: her newline ile biten satır bir mesajdır
- Harici bağımlılık olarak yalnızca Tokio

## Çalıştırma

```bash
cargo run
```

Sunucu `127.0.0.1:8080` adresini dinler.

## Manuel Chat Doğrulaması

1. Bir terminalde sunucuyu başlatın:

   ```bash
   cargo run
   ```

2. İki veya üç ayrı terminalde istemci açın:

   ```bash
   nc localhost 8080
   ```

   `localhost` çalışmazsa `127.0.0.1` kullanın:

   ```bash
   nc 127.0.0.1 8080
   ```

3. Bir istemcide mesaj yazıp Enter'a basın. Her mesaj newline ile bitmelidir.
4. Mesajın gönderen dahil tüm bağlı istemcilere ulaştığını kontrol edin.
5. İstemcilerden birini kapatın.
6. Kalan istemcilere yeni bir newline ile biten mesaj göndererek bağlantıların
   çalışmaya devam ettiğini doğrulayın.
7. Sunucuyu çalıştığı terminalde `Ctrl-C` ile durdurun.

Her mesaj newline ile bitmelidir. macOS ve Linux `nc` sürümleri farklı
davranabilir. `printf 'hello\n' | nc localhost 8080` gibi pipe tabanlı testler
mesaj gönderildikten hemen sonra bağlantıyı kapatabilir; broadcast ve bağlantı
temizliğini doğrulamak için interaktif terminaller daha güvenilirdir.

Ayrıntılı adımlar için [`docs/spec.md`](docs/spec.md) dosyasındaki Manuel
Doğrulama bölümüne bakın.

## Test ve Kontroller

```bash
cargo fmt --check
cargo check
cargo test
```

Testler, bir mesajın göndericiye ve diğer istemcilere ulaştığını; bir istemci
ayrıldıktan sonra kalan istemcilerin çalışmaya devam ettiğini doğrular.

## Proje Belgeleri

- [`docs/spec.md`](docs/spec.md): gereksinimler, protokol kuralları ve manuel
  doğrulama senaryosu
- [`docs/architecture.md`](docs/architecture.md): mevcut sistem mimarisi ve
  bağlantı yaşam döngüsü
- [`docs/tasks.md`](docs/tasks.md): tamamlanan ve gelecekteki milestone görevleri
- [`docs/decisions.md`](docs/decisions.md): alınan teknik ve mimari kararlar

## Gelecekte Eklenebilecek Geliştirmeler

Aşağıdaki özellikler mevcut kapsamda uygulanmamıştır:

- Kullanıcı adları
- Kimlik doğrulama
- Mesaj geçmişi
- Daha gelişmiş bir mesaj protokolü
- Graceful shutdown
