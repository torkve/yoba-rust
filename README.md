Yoba
====

Yoba is a modern enterprise programing language. It is powerful and can be used even by javascripters.

Yoba is based on Russian natural obscene language subset.
Do not teach it to any underage children, if you are the consummate hypocrite.

Anyway, 18+.

Syntax
------

Yoba supports arbitrary precision integers and variables.
Valid variable name is a sequence of alphanumeric characters starting with a letter.
Latin and cyrillic letters are supported.

Arithmetic expressions consist of variables and integers joined with `и` (addition) and/or `без` (substraction).
Example:

    сэмки и 42 и коты без 33

Program is a sequence of statements. Each statements starts with `чо` and ends with `йоба`.
Valid instructions are:

* define a variable:

    - `чо люблю VARIABLE йоба`

* assign an expression result to variable:

    - `чо VARIABLE это ARITHMETIC_EXPRESSION йоба`
    - `чо скамейка это 1 йоба`
    - `чо сэмки это пиво и скамейка`
    - `чо пиво это пиво и 1 йоба`

* inсrement variable by value:
    - `чо дай INTEGER VARIABLE йоба`
    - `чо дай VARIABLE INTEGER йоба`
    - `чо дай 1 скамейка йоба`
    - `чо дай семки 3 йоба`

* decrement variable by value:
    - `чо на INTEGER VARIABLE йоба`
    - `чо на VARIABLE INTEGER йоба`
    - `чо на 1 скамейка йоба`
    - `чо на семки 3 йоба`

* print variable:
    - `чо покажь VARIABLE йоба`
    - `чо покажь семки йоба`

* dump all variables:
    - `чо баланс нах йоба`

* empty operation:
    - `чо иди нахуй йоба`

* conditional operation (success branch is executed if variable greater or equal than integer):
    - `чо есть VARIABLE INTEGER тада SUCCESS_INSTRUCTION или FAILURE_INSTRUCTION йоба`
    - `чо есть INTEGER VARIABLE тада SUCCESS_INSTRUCTION или FAILURE_INSTRUCTION йоба`
    - `чо есть 3 семки тада покажь скамейки или покажь коты йоба`

* declare a procedure:
    - `чо усеки NAME это STATEMENTS йоба`
    - `чо усеки инкремент это чо семки это семки и 1 йоба йоба`
    - `чо усеки инкрементДо50 это чо есть 50 семки тада иди нахуй или семки это семки и 1 йоба йоба`

* call a procedure:
    - `чо хуйни NAME йоба`
    - `чо хуйни инкрементДо50 йоба`

Example
-------

Here is an example of a program calculating Fibonacci numbers:
```
чо люблю сэмки йоба
чо люблю пиво йоба
чо люблю яга йоба
чо люблю итерации йоба
чо пиво это 1 йоба
чо яга это 2 йоба

чо усеки результат это
    чо покажь итерации йоба
    чо покажь сэмки йоба
йоба

чо усеки фибоначчи это
    чо сэмки это пиво и яга йоба
    чо пиво это яга йоба
    чо яга это сэмки йоба
    чо итерации это итерации и 1 йоба
    чо есть итерации 50
    тада
        хуйни результат
    или
        хуйни фибоначчи
    йоба
йоба

чо хуйни фибоначчи йоба
```
