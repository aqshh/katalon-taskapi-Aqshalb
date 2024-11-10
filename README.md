# **Tugas Project untuk TAK pengujian API Automation**

------------------------------------------------
## **Informasi Dasar**
Aplikasi API testing ini adalah salah satu tugas project dari `Test Aplikasi Kamu (TAK)` dengan topik `Goes To QA Katalon Mobile Automation`, aplikasi ini berfokus pada cara kerja API dengan melakukan testing kepada end Point API.

## Panduan Aplikasi
### Requirement yang harus disiapkan:
* Menggunakan Bahasa Groovy based Katalon Studio v9
* Testing dilakukan pada End-Point API https://restful-booker.herokuapp.com/apidoc

## Ringkasan Hasil Testing
Pada pelaksanaan testing dilakukan testing pada `6 Test Case` dengan hasil:

```
6 Passed
0 Failed
0 Not Executed
```
Sehingga hasil pengujain adalah `100% Passed`

Pada pengujian, dibuat juga `Test Suite` agar dapat menjalankan beberapa test case secara bersamaan yaitu
```
TS_Get ID
TS_E2E
```
dan dibuat juga `Test Suites Collection` agar dapat menjalankan beberapa `Test Suite` secara bersamaan yaitu
```
TSC All
```

## End Point
End Point yang diuji pada project ini adalah:

### POST Token
Untuk mendapatkan kode token yang akan digunakan untuk semua testing, akan menghasilkan response code `200 OK` dan akan memberikan code token untuk digunakan.
```
https://restful-booker.herokuapp.com/auth
```

### GET Booking by ID
Untuk mendapatkan data booking berdasarkan ID
```
https://restful-booker.herokuapp.com/booking/1
https://restful-booker.herokuapp.com/booking/2
https://restful-booker.herokuapp.com/booking/3
https://restful-booker.herokuapp.com/booking/4
```
### POST Create Booking
Untuk menambahkan data booking, dengan memasukan pada `HTTP BODY` dengan atribute `TEXT` atau `JSON` akan menghasilkan response code `200 OK` dan menambahkan data
```
{
    "firstname":"Mamang",
    "lastname":"Racing",
    "totalprice":2500,
    "depositpaid":true,
    "bookingdates":{
      "checkin":"2024-11-09",
      "checkout":"2024-11-14"
    },
    "additionalneeds":"Race"
  }
}
```
### PUT Update Booking
untuk mengubah data booking berdasarkan id 
```
https://restful-booker.herokuapp.com/booking/1
```
Lalu memasukan data pada `HTTP BODY` dengan atribute `TEXT` atau `JSON` akan menghasilkan response code `200 OK` dan hasil perubahan data
```
{
  "firstname":"Ayyy",
  "lastname":"Yeee",
  "totalprice":2600,
  "depositpaid":true,
  "bookingdates":{
    "checkin":"2024-07-25",
    "checkout":"2024-01-26"
  },
  "additionalneeds":"Tea"
}
```
### DELETE Booking
untuk menghapus data bookingt berdasarkan id 
```
https://restful-booker.herokuapp.com/booking/1
```
Akan menghasilkan response code ```201 Created``` dan data terhapus.
