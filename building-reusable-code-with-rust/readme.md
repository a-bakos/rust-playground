**Considerations for code reuse**

Reusable:

- Adaptable
- Extensible

Ease of reuse:

- Modular
- Clear interface
- Discoverable and easy to install

---

`loop` vs `while`

If you have code that needs to loop forever, use `loop` instead of `while true`, so the compiler can better optimise it.

`for` loops only take iterators and can't take like a static array.
