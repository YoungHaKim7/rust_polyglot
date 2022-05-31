using CsharpMyApp.Calculator;
using CsharpMyApp.Models;
using System.Collections.Generic;
using NUnit.Framework;

namespace Expenses.Tests
{
    public class ExpensesCalculatorTests
    {
        [Test]
        public void ShouldReturnTotalForTwoExpenses()
        {
            var calculator = new ExpensesCalculator();
            var expenses = new List<Expense>() {
                new Expense { Id = 1, Description = "Lunch", Amount = 12.5m }),
                new Expense { Id = 1, Description = "Beer", Amount = 13.5m })
            };
        var total = calculator.CalculatorTotal(expenses);
        Assert.AreEqual(26m, total);
    }
}
}
