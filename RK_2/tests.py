#файл tests.py
from prod import *

def test_task1():
    PreResult1 = task1(Prepods, Courses)

    result_test_1 = True

    result_expect_1 = [
        (Prepods[0].FIO, Courses[0].name),
        (Prepods[0].FIO, Courses[1].name),
        (Prepods[1].FIO, Courses[2].name),
        (Prepods[3].FIO, Courses[4].name),
        (Prepods[3].FIO, Courses[5].name),
        (Prepods[3].FIO, Courses[6].name),
        (Prepods[4].FIO, Courses[7].name),
        (Prepods[2].FIO, Courses[3].name)
    ]
    print(result_expect_1)
    i = 0
    for (p, c) in PreResult1:
        print(p.FIO, "\t - \t", c.name)
        if(p.FIO != result_expect_1[i][0] or c.name != result_expect_1[i][1]):
            result_test_1 = False
            break
        i+=1
    # не можем сравнить в лоб из-за особенностей объектов в python (например просто распечатать объект нельзя)
    assert result_test_1
    
def test_task2():
    PreResult2 = task2(Prepods, Courses)

    result_test_2 = True

    result_expect_2 = [
        (Prepods[3].FIO, 3),
        (Prepods[0].FIO, 2),
        (Prepods[1].FIO, 1),
        (Prepods[2].FIO, 1),
        (Prepods[4].FIO, 1),
    ]


    i = 0
    for (p, c) in PreResult2:
        print(p.FIO, "\t - \t", len(c))
        if(p.FIO != result_expect_2[i][0] or len(c) != result_expect_2[i][1]):
            result_test_2 = False
            break
        i+=1

    assert result_test_2
    

def test_task3():
    PreResult3 = task3(Prepods, Courses, Cour_preps)

    result_test_3 = True

    result_expect_3 = [
        (Prepods[0], [Courses[0], Courses[1]]),
        (Prepods[1], [Courses[2]]),
        (Prepods[3], [Courses[4],Courses[5],Courses[6]])
    ]
    print(result_expect_3)

    i = 0
    for el in PreResult3:
        if(result_expect_3[i][0]==el[0] and result_test_3):
            j = 0
            for elC in el[1]:
                print(elC.name)
                if(elC!=PreResult3[i][1][j]):
                    result_test_3 = False # Одновременно флаг для внешнего цикла
                    break
                j+=1   
                   
        else:
            result_test_3 = False
            break
        i+=1  

    assert result_test_3