using System;
using System.Diagnostics;
using static System.Net.Mime.MediaTypeNames;

class Lab1
{
    static int EnterDouble() // доп функция для коррекного ввода чисел.
    {
        int result = 0;
        while (true)
        {
            //Console.Write("Введите число: ");
            string text = Console.ReadLine();
            if (int.TryParse(text, out result)) break;
            Console.Write("Некорректный ввод, попробуйте еще раз: ");
        }
        return result;
    }

    static void Main(string[] args){

        var sw = new Stopwatch(); // попробовал посчитать заодно и время выполнения программы
        if (args.Length != 0) sw.Start();
        int A, B, C;

        if (args.Length == 0) 
        // если не введены аргументы коммандной строки - вводим коэффициенты вручную
        {

            Console.Write("Введите коэффициент А: ");
            A = EnterDouble();

            Console.Write("Введите коэффициент B: ");
            B = EnterDouble();

            Console.Write("Введите коэффициент C: ");
            C = EnterDouble();
        }
        else
        {
            int.TryParse(args[0], out A);
            int.TryParse(args[1], out B);
            int.TryParse(args[2], out C);
        }

        Console.WriteLine($"Введено уравнение {A}x^4 + {B}x^2 + {C} = 0");

        int D = B * B - 4 * A * C;

        if (D < 0)
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Корней нет!");
            Console.ResetColor();
        }
        else if (D == 0)
        {
            double answer = -B / (2 * A);
            if (answer < 0)
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Корней нет!");
                Console.ResetColor();
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine($"x = {Math.Round(Math.Sqrt(answer),2)}");
                Console.ResetColor();
            }
        }

        else if (D > 0)
        {

            double answer1 = (-B - Math.Sqrt(D)) / (2 * A);
            double answer2 = (-B + Math.Sqrt(D)) / (2 * A);

            Math.Round(answer1, 3);
            Math.Round(answer2, 3);

            if (answer1 < 0 && answer2 < 0)
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Корней нет!");
                Console.ResetColor();
            }

            if (answer1 >= 0)
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine($"x1 = {Math.Round(Math.Sqrt(answer1), 2)}");
                Console.ResetColor();
            }
            if (answer2 >= 0)
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine($"x1 = {Math.Round(Math.Sqrt(answer2), 2)}");
                Console.ResetColor();
            }

        }

        if (args.Length != 0) {
            sw.Stop();
            Console.WriteLine(sw.Elapsed);
        }

        Console.ReadKey(); // чтобы exe-файл не закрывался сразу
    }
}
