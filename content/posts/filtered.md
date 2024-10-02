+++
title = '[Filtered]'
date = 2024-01-11T00:01:00Z
tags = ['youtube']
type = 'posts'
youtube = '4j5oDzRiXUA'
youtube_postfix = '/2'
summary = 'The puzzle itself got filtered...'
+++

<div><a href="/youtube/4j5oDzRiXUA/1.json">Older YouTube API JSON</a></div>
<div><a href="/youtube/4j5oDzRiXUA/0.json">Even older YouTube API JSON</a></div>

# Description

Old description:

> <code class="base64">
> L8gnk0GgYm9cCAA5t7wSIcwc669T+2TY/KlK4ATmQsnVrSY3PWrXAwUPcJiN1AugpkVwkDQARQydWkbLvs+4V0I08mSQKRsDinKZchmMlTJY8KCCS4ZDof1BxuCB7Uab6rAitGRYl+KgXqvROEbCWfb80nsDNaqo6wavnAVX5ld3nD6Ykl0vKIUUVNxuE42xDiMYuENg+tFLwsKcUzuw2KNZ+st46FBkZBniKP5jVQrqZzqAzgvcpHR63yMOZPkWMrVBHBwCRS31GRVx5qpzoB+0dkP0vX+YugYKIе9HvkEFJ440PCpMSd3ITK5Zmq/YJfAg5YyNpmRod3b2MVOfhX35lkjg41+4bidTo4H4d8sTiNz7YD74a5tWuzCj6BXax7ErPueqA7uRCcjaNXnGGrDLaFsEQkpFKWRmWm3hItAF5FiTqfB//7zj10iWCBDt3jJ1uNhrFLG7SX8kpvFyuw==
> <br/>
> [Key: **************G4]
> </code>

Old title: `"̷͔Ͱ̴΁̱ͭ;͉͌ͪ͊ͦͥ̀Ά͑΅ͳ΂΁̿Ϳ͉̻̽ͫͻ̸͕͸ͦͶ̳Ί͐͏͎͟΋̹͜΂·ΆͿ΄ͽ̓΁ͺͿ͸̾͟͞ͺ͹͸ͱ̷̶ʹ͖ͲͫͰͩͨΌ͎͍ͬͩͨͧ͟͠΃͚͙͖͛͢͝͞͝ͺ͚͙͑͐͘ʹ̷͓͒"`.

Old title (escaped):
> <code class="base64" style="line-break:inherit">
> \u0354\u0337\u0370\u0334\u0381\u036d\u0331\u037e\u034c\u036a\u034a\u0349\u0366\u0365\u0340\u0386\u0351\u0385\u0373\u0382\u0381\u033f\u037f\u033d\u0349\u033b\u036b\u037b\u0338\u0355\u0378\u0366\u0376\u0333\u038a\u0350\u034f\u034e\u035f\u038b\u0339\u035c\u0382\u0387\u0386\u037f\u0384\u037d\u0343\u0381\u037a\u037f\u0378\u033e\u035f\u035e\u037a\u0379\u0378\u0371\u0337\u0336\u0374\u0356\u0372\u036b\u0370\u0369\u0368\u038c\u036c\u034e\u034d\u0369\u0368\u0367\u0360\u035f\u0383\u035d\u0362\u035b\u035a\u0359\u035e\u035d\u0356\u037a\u035a\u0359\u0358\u0351\u0350\u0374\u0337\u0353\u0352
> </code>

New description:

> ```py
> def shift_characters(message):
>     return ''.join([char if char == ' ' else chr(ord(char) - 1) for char in message])
> 
> message = "hello world!"
> while True:
>     message = shift_characters(message)
>     print(message)
> ```

New title: `[Filtered]`

## Noise

The Hiyori image is always decomposed into 2 images with 0.5 opacity,
then each line of each image gets put to a certain position on-screen.
If the 2 images overlap, they blend.

The pattern is regular, but very complex. It's possible to undo some of
the mangling, as seen
[here](https://www.youtube.com/watch?v=eaWxFEaPlMk), but further
analysis of the mangling algorithm is currently ongoing.

## Solution 

The code in the new description hints at shifting the characters.
Indeed, all characters are from the same range, and when shifted down by
784 Unicode codepoints, the title becomes the following Malbolge
program:

```malbolge
D'`$q]!n<Z:9VU0vAucrq/o-9+[k(EhVf#z@?>O{)Lrwvotm3qjoh.ONjiha'&dFb[`YX|\>=YXWPOsMRKJINMFjJIHA@d'CB
```

This program prints "hello wo" and exits. However, if we use a Malbolge
generator to generate a program that prints "hello world!", we can
recover the missing piece at the end - `;:9]7<54981U543s+O)o-&+*#G4`.

The description says `[Key: **************G4]`, and the Malbolge program
indeed ends with `G4`. If we take the last 16 characters, we get the AES
decryption key.

[CyberChef link](https://gchq.github.io/CyberChef/#recipe=From_Base64%28'A-Za-z0-9%2B/%3D',true,false%29AES_Decrypt%28%7B'option':'UTF8','string':'U543s%2BO%29o-%26%2B*%23G4'%7D,%7B'option':'Hex','string':''%7D,'ECB','Raw','Raw',%7B'option':'Hex','string':''%7D,%7B'option':'Hex','string':''%7D%29&input=TDhnbmswR2dZbTljQ0FBNXQ3d1NJY3djNjY5VCsyVFkvS2xLNEFUbVFzblZyU1kzUFdyWEF3VVBjSmlOMUF1Z3BrVndrRFFBUlF5ZFdrYkx2cys0VjBJMDhtU1FLUnNEaW5LWmNobU1sVEpZOEtDQ1M0WkRvZjFCeHVDQjdVYWI2ckFpdEdSWWwrS2dYcXZST0ViQ1dmYjgwbnNETmFxbzZ3YXZuQVZYNWxkM25ENllrbDB2S0lVVVZOeHVFNDJ4RGlNWXVFTmcrdEZMd3NLY1V6dXcyS05aK3N0NDZGQmtaQm5pS1A1alZRcnFaenFBemd2Y3BIUjYzeU1PWlBrV01yVkJIQndDUlMzMUdSVng1cXB6b0IrMGRrUDB2WCtZdWdZS0llOUh2a0VGSjQ0MFBDcE1TZDNJVEs1Wm1xL1lKZkFnNVl5TnBtUm9kM2IyTVZPZmhYMzVsa2pnNGwrNGJpZFRvNEg0ZDhzVGlOejdZRDc0YTV0V3V6Q2o2QlhheDdFclB1ZXFBN3VSQ2NqYU5YbkdHckRMYUZzRVFrcEZLV1JtV20zaGx0QUY1RmlUcWZCLy83emoxMGlXQ0JEdDNqSjF1TmhyRkxHN1NYOGtwdkZ5dXc9PSA)

## Answer

> \[FILTERED\] ALWAYS TELLS ME SHE WANTS TO BE A VTUBER ONE DAY. SHE'S OBSESSED WITH THEM AND EVEN HAS HER OWN CHARACTER THAT SHE WANTS TO DEBUT AS. THAT'S ALL SHE EVER TALKS ABOUT. SHE SPEND MORE TIME IN THE \[FILTERED\] INSTEAD OF PLAYING OUTSIDE WITH EVERYONE ELSE. A TRUE INTROVERT... WE ARE IN MORE WAYS SIMILAR THAN I LIKE TO IMAGINE...

