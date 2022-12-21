import abc
from abc import ABCMeta


class Ast(metaclass=ABCMeta):
    def __init__(self, name: str):
        self.name = name

    @abc.abstractmethod
    def eval(self) -> int: ...

    def find(self, name: str):
        return self.name == name

    @abc.abstractmethod
    def find_humn_equal_to(self, const_value: int): ...


class Binary(Ast, metaclass=ABCMeta):
    def __init__(self, name: str, a: Ast, b: Ast):
        self.a = a
        self.b = b

        super().__init__(name)

    def find(self, name: str):
        return super().find(name) or self.a.find(name) or self.b.find(name)


class Add(Binary):
    def __init__(self, name: str, a: Ast, b: Ast):
        super().__init__(name, a, b)

    def eval(self) -> int:
        return self.a.eval() + self.b.eval()

    def find_humn_equal_to(self, const_value: int):
        if self.name == "humn":
            return const_value

        if self.a.find("humn"):
            local_const_value = self.b.eval()
            other = self.a
        else:
            local_const_value = self.a.eval()
            other = self.b

        return other.find_humn_equal_to(const_value - local_const_value)


class Mul(Binary):
    def __init__(self, name: str, a: Ast, b: Ast):
        super().__init__(name, a, b)

    def eval(self) -> int:
        return self.a.eval() * self.b.eval()

    def find_humn_equal_to(self, const_value: int):
        if self.name == "humn":
            return const_value

        if self.a.find("humn"):
            return self.a.find_humn_equal_to(const_value // self.b.eval())
        else:
            return self.b.find_humn_equal_to(const_value // self.a.eval())


class Div(Binary):
    def __init__(self, name: str, a: Ast, b: Ast):
        super().__init__(name, a, b)

    def eval(self) -> int:
        return self.a.eval() // self.b.eval()

    def find_humn_equal_to(self, const_value: int):
        if self.name == "humn":
            return const_value
        elif self.a.find("humn"):
            return self.a.find_humn_equal_to(const_value * self.b.eval())
        else:
            return self.b.find_humn_equal_to(self.a.eval() // const_value)


class Sub(Binary):
    def __init__(self, name: str, a: Ast, b: Ast):
        super().__init__(name, a, b)

    def eval(self) -> int:
        return self.a.eval() - self.b.eval()

    def find_humn_equal_to(self, const_value: int):
        if self.name == "humn":
            return const_value

        if self.a.find("humn"):
            return self.a.find_humn_equal_to(const_value + self.b.eval())
        else:
            return self.b.find_humn_equal_to(-const_value + self.a.eval())


class Num(Ast):
    def __init__(self, name: str, a: int):
        self.a = a

        super().__init__(name)

    def eval(self) -> int:
        return self.a

    def find_humn_equal_to(self, const_value: int):
        return const_value

def main():
    inp = [i for i in open("data.in").read().split("\n")]
    parts = {}
    for i in inp:
        name, rest = i.split(":")

        if "+" in rest:
            left, right = rest.split("+")
            node = Add(name, left.strip(), right.strip())
        elif "-" in rest:
            left, right = rest.split("-")
            node = Sub(name, left.strip(), right.strip())
        elif "*" in rest:
            left, right = rest.split("*")
            node = Mul(name, left.strip(), right.strip())
        elif "/" in rest:
            left, right = rest.split("/")
            node = Div(name, left.strip(), right.strip())
        else:
            node = Num(name, int(rest.strip()))

        parts[name] = node

    for i in parts.values():
        if isinstance(i, Binary):
            i.a = parts[i.a]
            i.b = parts[i.b]

    print(parts["root"].eval())

    const = parts["root"].b if parts["root"].a.find("humn") else parts["root"].a
    var = parts["root"].a if parts["root"].a.find("humn") else parts["root"].b
    assert not const.find("humn")

    const_value = const.eval()
    v = var.find_humn_equal_to(const_value)
    print(v)


if __name__ == '__main__':
    main()
