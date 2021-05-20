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

We can take an iterator to generate another iterator using some mathematical transformation. This is called an iterator adaptor.
Example: [1,2,3] plus 1 => [2,3,4]

We can also take an iterator an output something else like a number or some aggregated results. This is called a consuming adaptor.
Example: sum of [1,2,3] => 6
