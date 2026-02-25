# Dokumentasi NES

Struktur Hardware NES : 

## CPU 
    NES menggunakan CPU berbasis oleh arsitektur prosessor 6502. Prosessor ini memiliki kecepatan sekitar 1.79 MHz. NES 
    menggunakan versi modified dari prosessor ini, di NES, prosessor ini tidak punya mode desimal. Mode desimal maksudnya 
    adalah mode angka basis 10 yang biasa digunakan manusia. Misal : dalam binary, angka 10 adalah 0000 1010, dalam binary 
    decimal mode, angka 10 adalah 0001 0000 (4 bit pertama merepresentasikan angka 1, 4 bit berikutnya 0, 1 dan 0 = 10)

    Komponen Inti :
    - A (8 bit) = register accumulator, dipakai untuk operasi matematika
    - X (8 bit) = register X, bisa dipakai untuk loop counter / mengakses array dengan offset
    - Y (8 bit) = sama seperti X

    Komponen Spesial : 
    - SP (8 bit) = stack pointer, menunjuk posisi teratas di stack memori
    - PC (16 bit) = program counter, menunjuk instruksi selanjutnya yang akan dieksekusi
    - P  (8 bit) = status register, menyimpan flag / status operasi dari operasi terakhir
        status register mempunyai 7 flag (bit) yang menyimpan informasi sebagai berikut, urut dari kiri ke kanan :
            - FLAG_N (Negative) :          [0b10000000]  jika hasil operasi negative (bit 7)
            - FLAG_V (Overflow) :          [0b01000000]  jika ada buffer overflow matematika (bit 6)
            - 1                          : [0b00100000]  bit 5 gk dipakai sebagai flag, angkanya di set default ke 1
            - FLAG_B (Break)    :          [0b00010000]  jika ada instruksi BRK yang menyebabkan interrupt (bit 4)
            - FLAG_D (Decimal)  :          [0b00001000]  mode desimal (bit 3)
            - FLAG_I (Interrupt Disable) : [0b00000100]  blokir interrupt (bit 2)
            - FLAG_Z (Zero)              : [0b00000010]  jika hasil operasi 0 (bit 1)
            - FLAG_C (Carry)             : [0b00000001]  untuk menyimpan sisa pembagian

    CPU tidak bisa membaca langsung dari memori / ram yang ada, untuk membaca / menulis, mengakses ppu,
    mengakses apu, dia perlu lewat Bus, nah dalam NES, cpu punya semacam virtual memory nya sendiri, virtual memory 
    bukanlah memori asli yang benar benar ada di hardware, melainkan ia adalah sebuah abstraksi memori yang digunakan 
    oleh cpu. Virtual memory ini punya rentang dari 0x0000 sampai 0xFFFF. Nah dari 0x0000 sampai 0xFFFF, dibagi lagi menjadi
    beberapa bagian : 
        - 0x0000 - 0x07FF (2KB)        = Dipakai untuk menyimpan internal ram, yaitu ram utama game untuk menyimpan 
                                         variabel nyawa, skor, dll
        - 0x0800 - 0x1FFF              = Mirror dari Internal RAM, bisa diabaikan saja 
        - 0x2000 - 0x2007 (8 Byte)     = PPU Register, adalah register interface untuk mengatur grafik 
        - 0x2008 - 0x3FFF              = Mirror dari register PPU, diabaikan saja 
        - 0x4000 - 0x4017 (24 Byte)    = APU dan I/O 
        - 0x4018 - 0x401F              = Disabled, tidak dipakai 
        - 0x4020 - 0x5FFF              = Area untuk expansion ROM
        - 0x6000 - 0x7FFF              = SRAM (Save RAM), untuk menyimpan save data 
        - 0x8000 - 0xFFFF              = PRG-ROM

    Kenapa ada Mirror RAM dan Mirror PPU, untuk kasusnya mirror ram, itu karena dari cpu nya sendiri sebenernya udah nyediain
    15 pin yang bisa disambungin ke ram, cuman disini nintendo pingin motong biaya produksi, jadi dia beli ram yang murah,
    yang support cuman 11 pin (0-10), jadi ada 5 pin (yang mewakili address 0x0800 - 0x1FFF) yang nggak nyambung ke ram nya.

    Kenapa mulai mirrornya dari register 0x0800? Karena kalau kita lihat binernya : 
    0000 1|000 0000 0000

    nah kan si ram cuman support 11 pin, jadi dia cuman baca 11 bit doang dari kanan (little endian), mangkannya waktu di
    0x0800, itu angka satu ke kiri gk kebaca, dia kebacanya 000 0000 0000 doang atau 0x0000

    Cpu punya beberapa opcode / instruksi yang bisa dijalankan, penjelasan lebih lengkapnya akan dijabarkan di bagian opcode 
    
## PPU :
    Picture Processing Unit, adalah bagian dari hardware NES yang memproses segala sesuatu yang berkaitan dengan grafik.
    PPU ini punya beberapa register, antara lain : 
        - PPUCTRL (Write Only)
          Ini adalah tempat CPU memberi tahu bagaimana PPU harus bersikap. Diakses oleh CPU dengan menulis ke alamat 0x2000. Register ini berukuran 8 bit dengan deskripsi bit
          sebagai berikut : 
             - Bit 7 (V) : NMI Enable, untuk mengaktifkan NMI (*Non Maskable Interrupt*). Jika bit ini aktif, maka GPU bisa mengirim Non Maskable Interrupt ke CPU.
             - Bit 6 (P) : PPU Master / Slave, Ini adalah sistem untuk hardware yang punya 2 PPU, biar 2 PPU nya bisa bekerja sama untuk menghasilkan sebuah gambar di TV
                           NES hanya punya 1 buah PPU, sehingga fitur ini gak dipake
             - Bit 5 (H) : Ukuran Sprite (0: 8x8 pixel, 1: 8x16 pixel). Di dalam NES, ada aturan bahwa 
             - Bit 4 (B) : Background Tile Select, Alamat pattern table background (0: $0000, 1: $1000)
        - PPUMASK (Write Only)
          Digunakan untuk filter layar, bisa digunakan untuk menyalakan atau mematikan render background dan sprite. Diakses oleh CPU
          dengan menulis ke alamat 0x2001
        - PPUSTATUS (Read Only)
          Digunakan oleh CPU untuk mengetahui kondisi PPU. Diakses oleh CPU dengan membaca ke address 0x2002
        - OAMADDR (Write Only)
          PPU memiliki memori khusus untuk sprite bernama OAM (Object Attribute Memory). Register ini digunakan untuk menentukan alamat 
          mana di dalam OAM yang ingin diakses. Diakses oleh CPU dengan menulis ke address 0x2003
        - OAMDATA (Read / Write)
        - PPUSCROLL (Write Only - Twice)
          Register ini yang membuat game NES bisa scrolling (berjalan ke samping atau keatas)
        - PPUADDR (Write Only - Twice)
          Untuk menentukan alamat vram mana yang ingin diisi data. Karena Memori PPU (Vram) itu 16 bit, tapi jalu dari cpu cuma 8 bit, cpu harus menulis 2 kali ke register ini
        - PPUDATA

## Opcode : 
    Opcode adalah instruksi yang bentuknya binary, dan bisa dipahami oleh CPU dari NES.
    Berikut adalah list - list opcode yang ada di NES : 
    - SEI (Set Interrupt Flag)
      Adalah instruksi untuk mengubah bit flag status I (interrupt) menjadi 1 *(lihat penjelasan tentang register status di bagian CPU)*
      Opcode : 78
      Ukuran opcode: 1 byte
      contoh kodingan assembly : SEI
      
    - CLD (Clear Decimal Mode)
      Adalah instruksi untuk mengubah bit flag status D (decimal mode) menjadi 0 *(lihat penjelasan tentang register status di bagian CPU)*
      Opcode : D8
      Ukuran opcode : 1 byte
      contoh kodingan assembly : CLD
      
    - LDA (Load address A)
      Adalah instruksi untuk memasukkan angka kedalam register A (baca tentang register di bagian CPU). Untuk setiap operasi LDA, cpu juga akan mengecek hasil register A ini
      jadi negatif atau nol. Jika hasilnya negatif atau 0, maka bit flag status N (Negative) dan Z (Zero) akan diubah menjadi 1. Jika tidak negatif atau 0, value kedua bit flag
      tersebut akan diganti menjadi 0
      Ada beberapa jenis : 
        - LDA Immideate : Ambil byte berikutnya, taruh di register A
          Opcode : A9
          Ukuran opcode : 2 byte
          Contoh kode assembly : LDA #$80 [A9 80]
          Artinya : Masukkan angka 80 kedalam register A
          
        - LDA Absolute : Ambil byte dari alamat yang ditunjuk oleh 2 byte berikutnya
          Opcode : AD
          Ukuran opcode : 3 byte
          Contoh kode assembly : LDA $8000 [AD 00 80] (Disini 80 dan 00 nya terbalik karena CPU NES pakai sistem little endian)
          Artinya : Ambil byte dari alamat dengan address $8000, lalu simpan valuenya di register A
          
        - LDA Zero Page : Ambil byte dari ram bagian *zeropage* di alamat yang ditunjuk 1 byte berikutnya, alamat tersebut punya rentang antara $0000 - $00FF
          Opcode : A5
          Ukuran opcode : 2 byte
          Contoh kode assembly : LDA $10
          Artinya : Ambil byte dari alamat ram dengan address $10 ($0010), lalu masukkan ke register A



## Bus : 
    Secara fisik, bus adalah kumpulan kabel yang menghubungkan CPU dengan komponen lain, disini kita emulasikan BUS sebagai
    struct yang isinya semua komponen fisik NES

    CPU sifatnya buta, ia cuma tahu
        - "Saya mau baca alamat 0x2002"
        - "Saya mau tulis angka 0x80 ke alamat 0x2000"

    Bus adalah pihyak yang tahu 
        - "Alamat 0x2002 itu artinya dia mau cek status PPU (grafik)"
        - "Alamat 0x2000 itu artinya dia mau ngontrol PPU"

    Di dalam bus, kita definisikan komponen berikut : 
        - ram 
            - ukurannya 2kb
            - digunakan untuk menyimpan data
        - ppu (Picture Processing Unit)
            - chip terpisah yang mengurus grafik
        - apu (Audio Processing Unit)
            - chip yang mengurus suara
        - rom 
            - berisi semua kode program (PRG-ROM) dan data grafik (CHR-ROM)
        - controller
            - data tombol yang ditekan pemain

## ROM / Cartridge : 
    di dalam Cartridge, dibagi lagi jadi beberapa bagian : 
        1. Header (16 Byte): Metadata
            - Byte 0 - 3: Signature NES
            - Byte 4: Ukuran PRG-ROM (dalam satuan 16KB)
            - Byte 5: Ukuran CHR-ROM (dalam satuan 8KB)
            - Byte 6: Flag mapper
                Dalam byte 6, terdiri dari beberapa komponen bit : 
                    - Bit 0     : mirroring*
                    - Bit 1     : battery 
                    - Bit 2     : trainer, jika value 1 maka ada trainer, jika 0 nggak
                    - Bit 3     : four screen vram
                    - Bit 4 - 7 : mapper lower nibble
            - Byte 7: Flag mapper 2
                Dalam byte 7, terdiri dari beberapa komponen bit :
                    - Bit 0 - 1 : vs unisystem / playchoice
                    - Bit 2 - 3 : NES 2.0 indicator, kalau valuenya 10, maka pakai format NES 2.0
                    - Bit 4 - 7 : mapper upper nibble
            - Byte 8 - 15: Info lain (mirroring, dll)
        2. Trainer (Opsional 512 byte) : Untuk main game bajakan
        3. PRG-ROM: Kode program (Ukurannya sudah pasti kelipatan 16KB), size dari PRG-ROM adalah 16KB dikali
                    angka yang ada di Byte 4 di dalam header. Di dua byte terakhir PRG-ROM, ada yang namanya 
                    reset vector, reset vector ini akan digunakan oleh cpu untuk menentukan titik awal kode game mulainya
                    dari mana
        4. CHR-ROM: Data grafik (Ukurannyta sudah pasti kelipatan 8KB), size dari CHR-ROM adalah 8KB dikali 
                    angka yang ada di byte 5 di dalam header

    ROM asli dari nintendo sebenarnya gk punya header, header ini hanya ada untuk file dengan format .nes yang biasa dipakai
    di emulator. 

    Di dalam ROM / Cartridge yang asli, juga terdapat yang namanya mapper, mapper ini berguna untuk game yang ukuran cartridge 
    nya besar. Jadi ceritanya, si cpu ini cuma punya kabel / pin yang bisa akses 32 KB ke ROM / Cartridgenya. Buat game yang
    ukurannya diatas 32 KB, di Cartridgenya ada chip kecil yang namanya mapper, mekanismenya, nanti file gamenya bakal di bagi 
    jadi beberapa bagian yang ukurannya 16KB, terus nanti si mapper ini keep track, kita lagi ada di bagian yang mana. Mapper
    sendiri ada banyak jenisnya : 

        - Mapper 0 (NROM)
          Tidak ada chip mapper di dalam ROM, dipakai untuk game yang ukurannya< 32KB
        
        - Mapper 2 (UxROM)
          Biasa dipakai game action platformer. Address CPU dibagi jadi 2 bagian: 
            $8000 - $BFFF: Switchable bank (16KB). Ini adalah bagian yang bisa diganti ganti
            $C000 - $FFFF: Fixed bank (16KB). Bagian ini selalu mengacu ke 16KB terakhir dari ROM / Cartridge

        - Mapper 1 (MMC1 / SxROM)
          Biasa dipakai di game RPG dan open world, lebih canggih dari mapper2:
            - Switchable, bisa ganti bank PRG dan CHR
            - Ukuran bank bisa diatur, dari mode 16KB atau 32KB. 
            - Ukuran Fixed bank bisa diatur, bisa dibawah $8000 atau di atas $C000
            - Mendukung Save RAM

        - Mapper 3 (CNROM)
          Mirip dengan NROM atau UxROM untuk PRG, tapi dia bisa switch grafik  (CHR)
            - Ukuran PRG biasanya fixed 32KB, tidak bisa di switch 
            - Ukuran CHR bisa switch 8KB bank. ini memungkinkan game mengubah seluruh tampilan grafik karakter secara instan 

        - Mapper 4 (MMC3 / TxROM)
          mapper paling kompleks 
            - Ukuran bank size kecil (8KB)
            - Mapper ini bisa menghitung garis layar 

        - Mapper 7 (AxROM)
          Single screen mapping 
            - Switch bank dalam ukuran besar: 32KB 
            - Artinya, satu perintah switch akan mengubah seluruh area CPU ($8000-$FFFF)
            - Mirroring: mendukung single screen
            
### Istilah Istilah Teknis 

#### NMI (Non Maskable Interrupt) : 
  Ada 2 kata penting disini : 
    - Interrupt : sebuah interupsi, bayangin lagi baca buku, terus ada telpon, pasti kita tandain bukunya sekarang ada di halaman berapa,
                  angkat telfon, lalu lanjut baca lagi di halaman terakhir yang udah kita tandain. Nah sistemnya mirip, di NES, interrupt biasa
                  dikirim ke CPU saat ada yang urgent, biar cpu stop memproses kodingan, lalu nanganin hal yang urgent ini
    - Maskable  : berarti bisa ditutup / diabaikan. CPU punya flag I (Interrupt Disable) di status registernya, jika bit flag ini nyala, maka segala 
                  interrupt yang dikirim bisa diabaikan
                  
  Nah, Non Maskable Interrupt adalah teknologi yang dimiliki oleh PPU, untuk mengirim sinyal interrupt ke CPU yang tidak bisa diabaikan, meskipun CPU sudah 
  mengset bit flag I (Interrupt) nya ke 1. NMI Biasa dikirim saat *V-Blank baru saja dimulai*
  
#### V-Blank
  V-Blank (Vertical Blanking Interval), adalah jeda waktu "istirahat" yang terjadi di antara akhir satu frame dan dimulainya frame berikutnya. Buat ngerti ini,
  kita harus tahu dulu gimana teknologi TV Tabung (CRT) bekerja.
  
  TV CRT Jaman dulu menggambar layar dengan menggunakan sebuah "Tembakan Elektron" yang bergerak dengan sangat cepat. Tembakan ini mulai dari pojok kiri atas,
  bergerak ke kanan, lalu turun ke baris kebawahnya. Proses ini terus diulang sampai elektron sampai di pojok kanan bawah layar. Setelah sampai di pojok kanan bawah, 
  elektron harus kembali lagi ke pojok kiri atas untuk mulai menggambar frame baru.
  
  *V-Blank adalah waktu yang digunakan oleh tembakan elektron tersebut untuk bergerak kembali dari bawah ke atas*
  
  V-Blank ini adalah waktu yang penting, karena inilah waktu yang aman bagi CPU untuk mengirim data grafis baru ke PPU dengan aman. Karena kalau CPU mengirim data grafis 
  PPU saat tidak dalam waktu V-Blank, gambar yang dihasilkan akan terlihat ngeglitch
