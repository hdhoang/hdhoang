%windir%\system32\Dism /delete-image /index:2 /imagefile:M:\sources\install.wim /checkintegrity

%windir%\system32\Dism /Export-Image /SourceImageFile:\x64.wim /SourceIndex:1 /DestinationImageFile:M:\sources\install.wim /checkintegrity

%windir%\system32\Dism /Export-Image /SourceImageFile:D:\sources\install.wim /SourceIndex:1 /DestinationImageFile:M:\sources\install.wim /checkintegrity

%windir%\system32\Dism /Get-ImageInfo /ImageFile:M:\sources\install.wim
