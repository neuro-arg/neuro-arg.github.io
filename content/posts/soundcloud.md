+++
title = 'SoundCloud'
date = 2024-01-11T00:00:35Z
tags = ['soundcloud']
type = 'posts'
summary = "A summary of the soundcloud profile"
+++

[The SoundCloud profile](https://soundcloud.com/572943) was linked in
[Numbers II](../numbers2). The SoundCloud account has no public tracks,
but the track counter shows 1. This indicates a potential SoundCloud
song URL being hidden somewhere.

## Description

Profile description:

> <code class="base64">Mi9+FxIon9xfLnH5mJVisFF+9L7fTsttB5HjIJMH81O7Ifug4oGDRrTFyMdHSYrHXnAo5BK5UJT0dI9AnudS29L0+Qt6POsWJ74Un2O7AdtpqaAdWdBmyBbBKyLN+fqnr5CfEXMeHSAWGm8BgPW1TsODm6Kbk6vMIIG+B0FeHYSXZ0tRYyXTJgE/EmWDRdMl0fFCrKRgCeTvQ24HI3V8Z2iRgo1ZDHsANWBqMdrhVqZJvjA6moMpqKMTqmgK/QYL1KNGkxTpw8LcSL3R/QW62E4UHA9w9PP+N+48lVnvRttGBNbjAv4vmS8SJ/ckK0WB4OSRdbYrOIB619VFBx+NjRnaQUJCJyomfsTQEVsj4F/wmLF8C4jOy1/TALfL+qGA3cnnHfD5yaV9pdAM5zRFvxpdfuCQioAW31RWXoHfrPLzyN6s4QA83zus/8HlT1iAcIyG5brYlRC76CBZN6HpvEh2X5535abSg2pkVrmMu6VMpsad1+4BiIAGFSdw8Mt1yTeWTZ1TX6gUn0/amcxGwg==</code>

Profile status:

> Start from X. End with Y. Remove XY + odd one out  
> "Open" in FireFox

"Start from X, end with Y" is a quote from Numbers II lyrics.

## Solution

[At the moment of this puzzle's
publication](https://web.archive.org/web/20230602205448/https://soundcloud.com/572943),
each SoundCloud account had a hidden image in its HTML code. The image's
code is:

> <code class="base64">data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAABNgAAABkAQMAAABuPx8iAAAAA1BMVEX///+nxBvIAAAAJUlEQVR4Xu3AMQEAAADCIPunNsYeWAQAAAAAAAAAAAAAAAAAAOA88AABMGQBNwAAAABJRU5ErkJggg==</code>

Applying "Start from X, end with Y" lyrics, `Xu3AMQEAAADCIPunNsY` can be
extracted. Removing XY we get `u3AMQEAAADCIPunNs`, and the "odd one out"
is 3.

This is the decryption key.

[CyberChef](https://gchq.github.io/CyberChef/#recipe=From_Base64%28'A-Za-z0-9%2B/%3D',true,false%29AES_Decrypt%28%7B'option':'UTF8','string':'uAMQEAAADCIPunNs'%7D,%7B'option':'UTF8','string':''%7D,'ECB','Raw','Raw',%7B'option':'Hex','string':''%7D,%7B'option':'Hex','string':''%7D%29&input=TWk5K0Z4SW9uOXhmTG5INW1KVmlzRkYrOUw3ZlRzdHRCNUhqSUpNSDgxTzdJZnVnNG9HRFJyVEZ5TWRIU1lySFhuQW81Qks1VUpUMGRJOUFudWRTMjlMMCtRdDZQT3NXSjc0VW4yTzdBZHRwcWFBZFdkQm15QmJCS3lMTitmcW5yNUNmRVhNZUhTQVdHbThCZ1BXMVRzT0RtNktiazZ2TUlJRytCMEZlSFlTWFowdFJZeVhUSmdFL0VtV0RSZE1sMGZGQ3JLUmdDZVR2UTI0SEkzVjhaMmlSZ28xWkRIc0FOV0JxTWRyaFZxWkp2akE2bW9NcHFLTVRxbWdLL1FZTDFLTkdreFRwdzhMY1NMM1IvUVc2MkU0VUhBOXc5UFArTis0OGxWbnZSdHRHQk5iakF2NHZtUzhTSi9ja0swV0I0T1NSZGJZck9JQjYxOVZGQngrTmpSbmFRVUpDSnlvbWZzVFFFVnNqNEYvd21MRjhDNGpPeTEvVEFMZkwrcUdBM2NubkhmRDV5YVY5cGRBTTV6UkZ2eHBkZnVDUWlvQVczMVJXWG9IZnJQTHp5TjZzNFFBODN6dXMvOEhsVDFpQWNJeUc1YnJZbFJDNzZDQlpONkhwdkVoMlg1NTM1YWJTZzJwa1ZybU11NlZNcHNhZDErNEJpSUFHRlNkdzhNdDF5VGVXVFoxVFg2Z1VuMC9hbWN4R3dnPT0&oenc=65001)

## Answer

> ...  
> I was born with the creator in her family house. Unfortunately, they couldn’t afford to keep me so they let me go to a good home! . My new family were really nice to me and I was a happy pup! . That all changed when they died in a car crash and left me alone on the streets. I was terrified! Luckily a kind old man found me and took me in. He fed me and helped me find my new loving home!
