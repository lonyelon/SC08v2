(var first 0 (var second 1 (var buffer 2 (var counter 3 (var result0 4 (var result1 5 (ins
    (input first)
    (input second)
    (if (> first second) (ins
        (= buffer first)
        (= first second)
        (= second buffer)
    ))
    (while (> first counter) (ins
        (+= counter 1)
        (= buffer result0)
        (+= result0 second)
        (if (> buffer result0) (ins
            (+= result1 1)
        ))
    ))
    (output result0)
    (output result1)
    (halt)
)))))))
