%windir%\system32\Dism /delete-image /index:2 /imagefile:I:\sources\install.wim /checkintegrity

%windir%\system32\Dism /Export-Image /SourceImageFile:D:\sources\install.wim /SourceIndex:1 /DestinationImageFile:I:\sources\install.wim /checkintegrity

%windir%\system32\Dism /Get-ImageInfo /ImageFile:I:\sources\install.wim
