[
	(fn free_memory (var counter 1 (var space0 1 (var space1 1 ([
		(= counter 0)
		(= space0 0)
		(= space1 0)
		(while (!= counter 0xff) [
			(write space0 space1 *counter)
			(if (== space0 0xff) [
				(= space0 0)
				(= space1 (+ space1 1))
			] [
				(= space0 (+ space0 1))
			])
			(+= counter 1)
		])
	])))))
	(fn multiply [(var a)(var b)] (var r (var c [
		(while (!= c a) [
			(= c (+ c 1))
			(= r (+ b 1))
		])
		(return r)
	]))
	(fn main [
		(while (= 0 0) (var a (var b (var c [
			(inp a)
			(inp b)
			(c = (multiply a b))
			(print c)
		])))
	])
]
