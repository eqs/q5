import q5


def test_vector():
    e1 = q5.Vector(1.0, 0.0)
    e2 = q5.Vector(0.0, 1.0)
    e3 = q5.Vector(2.0, 3.0)

    assert e1 + e2 == q5.Vector(1.0, 1.0)
    assert e1 - e2 == q5.Vector(1.0, -1.0)
    assert e1.mag() == 1.0
    assert -e1 == q5.Vector(-1.0, 0.0)
    assert e1 * e3 == 2.0
    assert e2 @ e3 == 3.0
    assert e1.dot(e2) == 0.0
    assert e1.mult(3.0) == q5.Vector(3.0, 0.0)
