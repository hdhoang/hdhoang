Thêm autounattend.xml vào đĩa cài windows 7

Nguồn: https://rwmj.wordpress.com/2010/11/04/customizing-a-windows-7-install-iso

MS cung cấp bộ WAIK để tự động hoá quá trình cài đặt Windows, tuy nhiên nó mang theo nhiều chức năng không cần thiết cho công việc đơn giản này.

Tạo tập tin trả lời bằng http://windowsafg.no-ip.org/, lưu lại trong máy. Phần key để trống để có thể lựa chọn trong quá trình cài. THẬN CẤU HÌNH XÓA DỮ LIỆU.

Giải nén đĩa cài w7, có thể `mount -o loop`, hoặc giải nén bằng p7zip :
```
  % 7z x ../7600.16385.090713-1255_x86fre_client_en-us_Retail_Ultimate-GRMCULFRER_EN_DVD.iso
```
Tìm cấu hình khởi động:
```
  % isoinfo -d -i ../7600.16385.090713-1255_x86fre_client_en-us_Retail_Ultimate-GRMCULFRER_EN_DVD.iso
  [..]
    Eltorito defaultboot header:
        Bootid 88 (bootable)
        Boot media 0 (No Emulation Boot)
        Load segment 0
        Sys type 0
        Nsect 8 <--- số cung
        Bootoff 2DF 735 <--- khoảng cách
  % dd if=../7600.16385.090713-1255_x86fre_client_en-us_Retail_Ultimate-GRMCULFRER_EN_DVD.iso of=w7-boot.img bs=2048 count=8 skip=735
```
Nhập tập tin trả lời:
```
  % cp ../autounattend.xml .
```
Bạn cũng có thể chọn sẵn phiên bản w7 bằng cách tạo tập tin `ei.cfg` phù hợp. Tạo đĩa mới:
```
  % genisoimage -o ../w7.iso -b w7-boot.img -no-emul-boot -c BOOT.CAT -iso-level 2 -udf -J -l -D -N -joliet-long -relaxed-filenames .
```
