package main

import (
	"fmt"
	"math"
)

type Shape interface {
	Area() float64
	Perimeter() float64
}

type RoundThing interface {
	Diameter() float64
}

type Rectangle struct {
	Length, Width float64
}

func (r Rectangle) Area() float64 {
	return r.Length * r.Width
}

func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Length + r.Width)
}

type Circle struct {
	Radius float64
}

func (c Circle) Perimeter() float64 {
	return 2 * math.Pi * c.Radius
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c Circle) Diameter() float64 {
	return 2 * c.Radius
}

func main() {
	var c = Circle{3}
	var r = Rectangle{4.0, 6.0}

	var shape Shape
	var roundThing RoundThing

	shape = c
	fmt.Printf("Shape Type = %T, Shape Value = %v\n", shape, shape)
	fmt.Printf("Area = %f, Perimeter = %f\n", shape.Area(), shape.Perimeter())

	roundThing = c
	fmt.Printf("round thing Diameter = %f, Shape Value = %v\n", roundThing.Diameter(), roundThing)

	shape = r
	fmt.Printf("Shape Type = %T, Shape Value = %v\n", shape, shape)
	fmt.Printf("Area = %f, Perimeter = %f\n", shape.Area(), shape.Perimeter())

}

