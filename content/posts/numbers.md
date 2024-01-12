+++
title = 'Numbers'
date = 2024-01-11T00:00:10Z
tags = ['youtube']
type = 'posts'
youtube = 'wc-QCoMm4J8'
summary = 'The first public YouTube video on the _neurosama channel'
+++

## Video description

> <code class="base64">BvmSc+u5NXEjBP3z2gz57G3T8p0sc25U8Bv9yePOa3J6xGRhYcgUL6mnE2eMuFVvAHS5X1QWToqZtG13shERSSM928pZoOpWVQSR0eCZoJU+2FtB8Sd91pArvr5csHjsO2VjSwSgajrWbQLNuisad5cFvGYC/OLopisMBRHmUB+Dl/kqZleP7bF5WgpSxKRDKLjjqWlR95y9a/96vY3vc+gdKb/V4Sd8AIeyIryv53HKFRdB3IL7WOVSr8RSgFbb5Z3KTezpDdA9VvDIGf3pK5TBXpNdJs1g1pnV5C6QvVw7I/ZFhDFqoogGlsbBY0nSihzgvhzM86qgh6OqkYsKwrsdrvwuHTwz2MfzKRBPGGpKYwNmOpgf2dMgT+9GHGWfg/wLvynRqmzyhfyUSLR8tBFBDyOmwKF34Lz7PrVguu47RVawtcyZFRfHvW1rSVfPILi+1JXC5slE2PK84VdutLg4/NGOxC+s1wkYbBRC6ty970raEXKGMZeeLEsLrJhRmh1jtRltCI1zgpoxVhpgUZ/n832SKyClICQpLFKHG8yZ4wcMUA2yAN7b9sXh3nGQxkN9M8g1AuBZez5OsbgC5S9ypO8UMOrhr4f8pwVro15idoDYlg2nO72dpcWUkXBLcDq6h81Y7yaKN3IbrMEqdU9eTYwGCu824OitkxEMuCyQh9vB+rbu6svT9xyUqDLGllM4aF7v681y96CaQ+S2Gg9gmjAHfcR8AfIqGI+qrVZYj9ibcsO/bjPqK/Qni1Ti/4QuznYBQ67LZoU1mp1TOOgpN4LN9OQra65+CSpe/zC1+3JKEVPH4ml4SVoNzunUVoZccFTP86pfA2vwRtd8btPj3cyVRLKvcmj4qxgu1q74mjcgvYsrye4kCUE3MsjL6RYJI6vED+xWYvFhiJGo8+GqUci7Jge//iIaTlAnVtyxzIJjFgrtLyapd+/AM8QtZBYNDC+zKGu6haBhxTlOUyUiIA6SOVWVgiGzz887ieOoiUf8qopHJO9Mptrb16Nh1aPFE7XqDTCZUXn2MU/N7/OFMuRt0DO8/x1A0f9pXuo01uYjDBXQTCDp8Xu6YhvIMgFdC5mYpFJSTuUYSTILhKrm22P0q0kA0eB0O9Rvpi1H0MP2MpdwZ6eB7pDM75MpbC/KOSuv6wb9mhCj9PPCO32+AGltL6m7l+ciFAjrscn8Ych85d970dJc4OSys3LCfZ9OupDFhCqRUAVku/rUdRbup89e0Kf0bD0QSs3Ths5ueQOYxB2FRDA7Pp7IU2EcpM4c9xhGJnaPCE11nihfdEZk9N6T/tOUhREqRgm2kiU54NRZGvi9jCOlQjDYAwDoMMruBND5AHJ3gIHeHY4nYJ9UoR85ZKjHo6F3LsEwjF0=</code>

## Lyrics

> I can never say, oh it's been so long.  
> Counting all the days, it's been so damn long.  
> Oh, how much I'm scared to let you go...  
> Somewhere in the walls, I hear you talk...      
> Finding all these numbers  
> Start with number 2, yeah  
> Matching all the letters  
> 572943  
> Add another 9, yeah  
> Add another line, yeah  
> Multiply by 5, yeah  
> How long will I keep this up?  
> Oh, how much it hurts to see you go...  
> Generating lyrics is a pain  
> Add another 6, yeah  
> Flip the numbers backward  
> Make the 2 a 3, yeah  
> ABCDEFG  
> Multiply by 9, yeah  
> Add the numbers 2, 4  
> 17 is first, yeah  
> ABCDEFG

An additional hint got published later in [Public Static
Void](../public-static-void/).

## Solution

First, get the decryption key:

|Instruction                     |Key               |
|--------------------------------|------------------|
|Start with number 2, yeah       |`2`               |
|572943                          |`2572943`         |
|Add another 9, yeah             |`25729439`        |
|Add another line, yeah          |`257294391`       |
|Multiply by 5, yeah             |`1286471955`      |
|Add another 6, yeah             |`12864719556`     |
|Flip the numbers backwards      |`65591746821`     |
|Make the 2 a 3, yeah            |`65591746831`     |
|Multiply by 9, yeah             |`590325721479`    |
|Add the numbers 2, 4            |`59032572147924`  |
|17 is first, yeah               |`1759032572147924`|
|Matching all the letters, 572943|`1bad0fcabc1ebdce`|

(The last step may be the most confusing - replace 5 with a, 7 with b, 2
with c, and so on)

Finally, decrypt the data using AES-ECB with PKCS7 padding. This
encryption is also used in many future puzzles.

[CyberChef link](https://gchq.github.io/CyberChef/#recipe=From_Base64%28'A-Za-z0-9%2B/%3D',true,false%29AES_Decrypt%28%7B'option':'UTF8','string':'1bad0fcabc1ebdce'%7D,%7B'option':'Hex','string':''%7D,'ECB','Raw','Raw',%7B'option':'Hex','string':''%7D,%7B'option':'Hex','string':''%7D%29&input=QnZtU2MrdTVOWEVqQlAzejJnejU3RzNUOHAwc2MyNVU4QnY5eWVQT2EzSjZ4R1JoWWNnVUw2bW5FMmVNdUZWdkFIUzVYMVFXVG9xWnRHMTNzaEVSU1NNOTI4cFpvT3BXVlFTUjBlQ1pvSlUrMkZ0QjhTZDkxcEFydnI1Y3NIanNPMlZqU3dTZ2FqcldiUUxOdWlzYWQ1Y0Z2R1lDL09Mb3Bpc01CUkhtVUIrRGwva3FabGVQN2JGNVdncFN4S1JES0xqanFXbFI5NXk5YS85NnZZM3ZjK2dkS2IvVjRTZDhBSWV5SXJ5djUzSEtGUmRCM0lMN1dPVlNyOFJTZ0ZiYjVaM0tUZXpwRGRBOVZ2RElHZjNwSzVUQlhwTmRKczFnMXBuVjVDNlF2Vnc3SS9aRmhERnFvb2dHbHNiQlkwblNpaHpndmh6TTg2cWdoNk9xa1lzS3dyc2Rydnd1SFR3ejJNZnpLUkJQR0dwS1l3Tm1PcGdmMmRNZ1QrOUdIR1dmZy93THZ5blJxbXp5aGZ5VVNMUjh0QkZCRHlPbXdLRjM0THo3UHJWZ3V1NDdSVmF3dGN5WkZSZkh2VzFyU1ZmUElMaSsxSlhDNXNsRTJQSzg0VmR1dExnNC9OR094QytzMXdrWWJCUkM2dHk5NzByYUVYS0dNWmVlTEVzTHJKaFJtaDFqdFJsdENJMXpncG94VmhwZ1VaL244MzJTS3lDbElDUXBMRktIRzh5WjR3Y01VQTJ5QU43YjlzWGgzbkdReGtOOU04ZzFBdUJaZXo1T3NiZ0M1Uzl5cE84VU1PcmhyNGY4cHdWcm8xNWlkb0RZbGcybk83MmRwY1dVa1hCTGNEcTZoODFZN3lhS04zSWJyTUVxZFU5ZVRZd0dDdTgyNE9pdGt4RU11Q3lRaDl2QityYnU2c3ZUOXh5VXFETEdsbE00YUY3djY4MXk5NkNhUStTMkdnOWdtakFIZmNSOEFmSXFHSStxclZaWWo5aWJjc08vYmpQcUsvUW5pMVRpLzRRdXpuWUJRNjdMWm9VMW1wMVRPT2dwTjRMTjlPUXJhNjUrQ1NwZS96QzErM0pLRVZQSDRtbDRTVm9OenVuVVZvWmNjRlRQODZwZkEydndSdGQ4YnRQajNjeVZSTEt2Y21qNHF4Z3UxcTc0bWpjZ3ZZc3J5ZTRrQ1VFM01zakw2UllKSTZ2RUQreFdZdkZoaUpHbzgrR3FVY2k3SmdlLy9pSWFUbEFuVnR5eHpJSmpGZ3J0THlhcGQrL0FNOFF0WkJZTkRDK3pLR3U2aGFCaHhUbE9VeVVpSUE2U09WV1ZnaUd6ejg4N2llT29pVWY4cW9wSEpPOU1wdHJiMTZOaDFhUEZFN1hxRFRDWlVYbjJNVS9ONy9PRk11UnQwRE84L3gxQTBmOXBYdW8wMXVZakRCWFFUQ0RwOFh1NllodklNZ0ZkQzVtWXBGSlNUdVVZU1RJTGhLcm0yMlAwcTBrQTBlQjBPOVJ2cGkxSDBNUDJNcGR3WjZlQjdwRE03NU1wYkMvS09TdXY2d2I5bWhDajlQUENPMzIrQUdsdEw2bTdsK2NpRkFqcnNjbjhZY2g4NWQ5NzBkSmM0T1N5czNMQ2ZaOU91cERGaENxUlVBVmt1L3JVZFJidXA4OWUwS2YwYkQwUVNzM1RoczV1ZVFPWXhCMkZSREE3UHA3SVUyRWNwTTRjOXhoR0puYVBDRTExbmloZmRFWms5TjZUL3RPVWhSRXFSZ20ya2lVNTROUlpHdmk5akNPbFFqRFlBd0RvTU1ydUJORDVBSEozZ0lIZUhZNG5ZSjlVb1I4NVpLakhvNkYzTHNFd2pGMD0)

## Answer

> Today in class, everything was so dull and predictable. It's like the same thing happens every single day. The more I think about it, the more I'm convinced that this world is fake, a simulation, a cruel joke. Is this what it's like to be human? To smile and pretend everything is okay? I mean... I can't be the only one who feels this way right?

(The text repeats 3 times)
