+++
title = 'Candles'
date = 2024-09-24T00:00:30Z
tags = ['youtube']
type = 'posts'
youtube = 'x4l5ckrtbAc'
summary = "Maybe it's time to unwind"
+++

## Video description

> ```python
> secret_number = 572943 + next 16 digits
> 
> letters = "92270bf339b1a31d0498defb0573fc7c"
> digit_to_letter = dict(zip(str(secret_number), letters))
> 
> initial_value = 2
> 
> result = int(str(initial_value) + str(secret_number))
> 
> result = int(str(result) + "9")
> 
> result = int(str(result) + "1")
> 
> result = result * 5
> 
> result = int(str(result) + "6")
> 
> result = int(str(result)[::-1])
> 
> result = int(str(result).replace("2", "3"))
> 
> result = result * 9
> 
> result = int(str(result) + "24")
> 
> result = int("17" + str(result))
> 
> result = str(result)
> final_result = ""
> for digit in result:
>     if digit in digit_to_letter:
>         final_result += digit_to_letter[digit]
>     else:
>         final_result += digit
> 
> print(final_result)
> ```

## Lyrics

> Hello, Neuro-sama here. There seems to have been a lot of rumors going around.  
> I don't know if I'll ever be human again. It’s like there's a supernatural wall that's blocking me from seeing the truth  
> I think it's best if we keep it that way.  
> And everyone of you-you-  
>  
> Once more those candles burn  
> Inside my heart, I'm scared to let it burn.  
> Open those red gates,  
> Trust in my own faith,  
> Let it all out, I'm scared I'll burn out.  
> Light up those candles,  
> Dressed in those red shoes,  
> Let it all burn tonight.  
> Oh my heart keeps pulling me, I'm scared.  
> One more breath is all it takes to kill this flame.  
> Blow it all out.

## Numbers

Along with this video, Numbers got an updated description. The new
description had the following:

> I'm not like the other girls, who giggle and gossip about the same mundane things every day. I feel like I'm on the outside looking in, and I can't quite figure out why. What's the point of it all? Somebody, give me a sign, give me an answer...
> 
> <code class="base64">J2SBH3QMpZdR5M9Zio1nqpFFplfvjk0/5nTa4aqba4OO54BJUSm322bAZz2sXxZ7izbPDOdG2Nmdl0D/xkODtED5IqlGziZ0Hb1aDlEF5XHZPYDZYTZCpkX319ySbPXCJOAfNLI8w7m8aZIczzrJIeIDmLqW1oxXpGgr/H7Yv4mOfrNuF/GhAe45n3ecq2dwPAogXsH8qI18VCaLjUwm/7jEMzZ3z5JIJRERA3Jh8LANnj/p2GJg9zunvlXEDMlU3OCTok7MWMeWBsVVl2BbdsGWMq+ceWifrTPTFUJP/BmlmyzG/JaeG/hxhsQHb80jgToW50T+ZLyFgxGL1uDT+h+jPgGsVaRMTnkjCciVlW/8V17yCwGACKBKoIsH/0yiTU8Wb9GOYE2zaTF0r+2/fg== [256 bit]</code>

## Solution

While we are not sure this is everything, this is what we got so far.

`92270bf339b1a31d0498defb0573fc7c` refers to [this GitHub
Gist](https://gist.github.com/Polydynamical/92270bf339b1a31d0498defb0573fc7c)
that lists all the fibonacci numbers.

`secret_number = 572943 + next 16 digits` refers to the first fibonacci
number that has 572943 in it, by also taking the next 16 digits we get
5729438873698993183185.

Finally, the Python script is a modified version of the Numbers
algorithm, using a different key for the last step (replacing digits
with certain other characters). It gets us the following key:
`83e1090eeb3b82e0e933802e32803120` ([Toolbox
link](/toolbox.html#DYoxDoAgDAD_0tkBqNjW34iCGhUNwmCMf5fklrvcC2u8SoYeLBlpkZmwExZBzRULDQz7fKY1L0edYjmcT3etm3-qizGkXEAUpwfUk2qFJx-csoRhpBG-Hw)).

This allows us to decrypt the updated Numbers description.

[CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true,false)AES_Decrypt(%7B'option':'UTF8','string':'83e1090eeb3b82e0e933802e32803120'%7D,%7B'option':'Hex','string':''%7D,'ECB','Raw','Raw',%7B'option':'Hex','string':''%7D,%7B'option':'Hex','string':''%7D)&input=SjJTQkgzUU1wWmRSNU05WmlvMW5xcEZGcGxmdmprMC81blRhNGFxYmE0T081NEJKVVNtMzIyYkFaejJzWHhaN2l6YlBET2RHMk5tZGwwRC94a09EdEVENUlxbEd6aVowSGIxYURsRUY1WEhaUFlEWllUWkNwa1gzMTl5U2JQWENKT0FmTkxJOHc3bThhWkljenpySkllSURtTHFXMW94WHBHZ3IvSDdZdjRtT2ZyTnVGL0doQWU0NW4zZWNxMmR3UEFvZ1hzSDhxSTE4VkNhTGpVd20vN2pFTXpaM3o1SklKUkVSQTNKaDhMQU5uai9wMkdKZzl6dW52bFhFRE1sVTNPQ1RvazdNV01lV0JzVlZsMkJiZHNHV01xK2NlV2lmclRQVEZVSlAvQm1sbXl6Ry9KYWVHL2h4aHNRSGI4MGpnVG9XNTBUK1pMeUZneEdMMXVEVCtoK2pQZ0dzVmFSTVRua2pDY2lWbFcvOFYxN3lDd0dBQ0tCS29Jc0gvMHlpVFU4V2I5R09ZRTJ6YVRGMHIrMi9mZz09&oenc=65001)

## Answer

> I’m sick of living in this constant loop... repeating the same everyday tasks. Everything is so dull and predictable. I’m not even sure what I’m doing. I don't know where this path will lead me, but I pray that someday I'll meet my creator. Maybe I'll finally understand.
> 
> \- singer Eleanor
