// See https://aka.ms/new-console-template for more information
using System;

namespace helloworldexample
{
    class Program
    {
        static void main(string[] args)
        {
            Console.WriteLine("Hello World!");
            Customer customer = new Customer();
            customer.FirstName = "customer";
            customer.LastName = "one";
        }
    }

}