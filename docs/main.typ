#import "@preview/cetz:0.1.1": *
#import "template.typ": *
#show: ieee.with(
  title: "Fundamentals of Quantitative Finance",
  abstract: [
    This paper explores the fundamentals of quantitive finance and capital market theory, introducing the underlying quantitative methods driving capital asset allocation decision-making. Using these methods as building blocks, this paper will model financial market infrastructure to simulate optimal decision-making market participants.
  ],
  authors: (
    (
      name: "Ryan Tate",
      // department: [Founder],
      organization: [Emergent Financial, LLC],
      location: [Silverdale, WA],
      email: "ryan.tate@emergent.financial"
    ),
  ),
  index-terms: ("finance", "quantitative", "capital", "markets"),
  bibliography-file: "refs.bib",
)

= Introduction


= Quantitative Methods

== *Time-Value of Money*

Money is productive over time. Asset allocators forgo an opportunity cost when they choose to invest in one project over another. Determining where to allocate capital factors in other options that would yield a return. The `discount rate` is referred to as the `cost of capital`. 

An obvious requirement of an investor is that the expected rate of return exceeds the discount rate or the cost of capital. 

For example, if a corporation is borrowing money to finance a project, the discount rate or the cost of capital is at least equal to the cost of borrowing the money to finance the project. 

In another example, an equity investor may decide to use a long-term average private equity return as the discount factor for investing in a private offering.

The discount rate is the rate of return an investor expects to receive in exchange for deferred payment.

\
=== Discount Factor

The discount factor $D_f$ is the periodic rate of return $r$ discounted over $n$ periods. 

$ D_f = 1 / (1 + r)^n $

\
=== Present Value

The present value $"PV"$ is equal to the future cash flows $"CF"$ over $n$ periods discounted at a rate of $r$. $ "PV" = "CF" * D_f = "CF" / (1+r)^n $

\
=== Future Value

The future value $"FV"$ is equal to an initial investment cash flow $"CF"_0$ that is expected to return a rate of $r$ over $n$ periods. 




$ "FV" = "CF"_0(1+r)^n $

\
=== Present Value of an Annuity (Fixed Cash Flows)

The present value $"PV"$ of an annuity is equal to a series of fixed cash flows $"CF"$ received over $n$ periods given a discount rate of return of $r$. $ "PV" = sum_(t=1)^n "CF"/(1+r)^n $

\
=== Present Value of Variable Cash Flows

The present value $"PV"$ of a series of variable cash flows $"CF"$ received over $t$ time periods $(t_1, t_2, t_3,...,t_n)$ given a discount rate of return $r$ can be expressed as:

$ "PV" = "CF"_1/(1+r) + "CF"_2/(1+r)^2 + ... + "CF"_n/(1+r)^n $

Or,

$ "PV" = sum_(t=1)^n "CF"_t/(1+r)^t $

\
=== Net Present Value of an Investment

The net present value of $"NPV"$ an investment is equal to the initial cash outlay $"CF"_0$, which is a negative cash flow, or the cost of the investment plus the future expected cash flows $"CF"$ from the investment.

$ "NPV" = -"CF"_0 + sum_(t=1)^n "CF"_t/(1+r)^t $

== *Bond Pricing Model*

\
=== Coupon Payment

The coupon payment $"Coupon Payment"$ is the `par` or `face value` of the bond $"Par"_v$ multiplied by the interest rate $r$ of the bond. $ "Coupon Payment" = "Par"_v * r $

\
=== Present Value of a Bond

The present value of a bond $B_v$ can be calculated by modifying the net present value formula, where instead of an initial cash outlay, the repayment of the `par` value ($"Par"_v$) of the bond (`face value`) is added to the future coupon payments ($"CP"$) (i.e. cash flows) over $t$ time periods given a discount rate of $r$.

$ B_v = "Par"_v + sum_(t=1)^n "CP"_t/(1+r)^t $

\
=== Yield-to-Maturity (YTM)

The yield-to-maturity ($"YTM"$) for a bond can be calculated by taking the square root of the par value of the bond divided by the current price of the bond, minus one. The YTM represents the discount rate that makes the current market price of a bond equal to its calculated present value.

$ "YTM" = root(t, "Par"_v/("Par"_v + sum_(t=1)^n "CP"_t/(1+r)^t)) - 1 $

Or,

$ "YTM" = root(t, "Par"_v/B_v) - 1 $

Where $B_v$ is the `Bond Current Value` using the `Bond Pricing Model` formula from above.

\
=== Macaulay's Duration

Duration ($D$) is the weighted average of the time until each payment proportional to the present value of each payment, using the yield-to-maturity ($"y"$) as the discount rate, periodic coupon payment cash flows ($"CP"_t$), par value ($"Par"_v$), and the current bond value ($B_v$). Macaulay's Duration is a measure of a bond's sensitivity to changes in interest rates.

$ D = [ ( (1"CP")/(1+"y")^1 + (2"CP")/(1+"y")^2 + ... + t("CP" + "Par"_v)/(1+"y")^n ) / B_v ] $

Or,

$ D = (sum_(t=1)^n (t times "CP")/(1+y)^t + (n times "Par"_v)/(1+y)^n) / B_v $

where

$t$ = Respective time period

$"CP"$ = Periodic coupon payment

$y$ = Periodic yield

$n$ = Total number of periods

$"Par"_v$ = Maturity (Par/Face) value

$B_v$ = Current bond balue
​

\
=== Modified Duration

Modified duration ($"MD"$) is an extension of Macaulay's Duration, and measures the average cash-weighted term to maturity of a bond.

$ "MD" = D / (1 + (y/n)) $

== *Common Yield Measurements*

\
=== Holding Period Yield (HPY)

$ "HPY" = (P_1 - P_0 + D_1) / P_0 $

\
=== Effective Annual Yield (EAY)

$ "EAY" = (1 + "HPY")^(365/t) - 1 $ 

\
=== Bank Discount Yield ($r_"BD"$)

$ "r"_"BD" = ("Par"_v - "Sale Price") / "Par"_v times 360/t $

where

$"Par"_v$ = Par value of the bond

$t$ = Days to Maturity

\
=== Money Market Yield (CD Equivalent Yield) ($r_"MM"$)

$ "r"_"MM" = "HPY" times 360/t $

Or,

$ "r"_"MM" = (360r_"BD") / (360 - (t)(r_"BD")) $

\
=== Internal Rate of Return ($"IRR"$)

The Internal Rate of Return ($"IRR"$) is the discount rate that makes the net present value ($"NPV"$) of an investment equal to zero.

$ 0 = "NPV" =sum_(t=1)^n "CF"_t/(1+"IRR")^t - "CF"_0 $

The $"IRR"$ must be calculated in reference to the periodic cash flow. An iterative solution is required to solve this equation, requiring adjusting the $"IRR"$ until the $"NPV"$ is equal to zero. This method is considered a _"guess-and-check"_ algorithm.

== *Statistical Methods*

\
=== Population Mean

$ mu = (sum^N_"i=1" X_i) / N $

\
=== Sample Mean

 $ overline(X) = (sum^n_"i=1" X_i) / n $

 where $n <= N - 1$



\
=== Geometric Mean

The geometric mean ($overline(X)_G$) is often used to average rates of return over time, particularly useful for time-series portfolio holding returns.

$ overline(X)_G = root(t, (1 + r_1)(1 + r_2)...(1 + r_t)) - 1 $

Or,

$ overline(X)_G = [ product^t_(t=1)(1+r_t) ]^(1/t) - 1 $

\
=== Weighted Mean

$ overline(X)_W = sum^n_(i=1)w_i X_i $

where $sum_i w_i = 1$

\
=== Harmonic Mean

The harmonic mean ($overline(X)_H$) is an inverse weighted-average of an observation proportional to its magnitude. One of it's limited applications is cost-averaging of investments. 

$ overline(X)_H = n / (sum^n_(i=1)1 / X_i) $



_Note: $overline(X)_H$ should only be used at discrete time periods. Varying capital allocations over multiple time periods should use the weighted mean $overline(X)_W$ formula._

$  $

\
=== Quantiles

Quantiles are used to describe what fraction of a distribution is at or below a certain point. For example, quartiles divide the distribution into quarters to find the 25%, 50%, 75%, and 100% percentiles.

$
"Quartiles" = n/4
$

$
"Quintiles" = n/5
$

$
"Deciles" = n/10
$

$
"Percentiles" = n/100
$

where $X_i <= X_(i+1) "(Ascending order)"$ 

The index at which a percentile of the distribution is at or below ($i_(y)$) within an ascending distribution can be calculated as:

$ i_y = (n + 1) (y / 100) $

when $i_y in.not [WW] ("Whole Number Set"), i_y >= 0$

Use _linear interpolation_ between the ceiling and floor of $i_y$ when the index value is not a whole number to find the value ($P_y$) at or below which $y$ percent of the distribution lies.

$ P_y = X_floor(i_y) + (X_i_y - X_floor(i_y)) (X_ceil(i_y) - X_floor(i_y)) $

where 

$ceil(i_y) in [WW] \ floor(i_y) in [WW]$

\
=== Range

$ "Range" = X_"max" - X_"min" $

\
=== Deviation (Distance) from the Mean

$ X_i - overline(X) $

\
=== Variance (Population & Sample)

Population: $ sigma^2 = (sum^N_(i=1) (X_i - mu)^2) / N $
Sample: $ s^2 = (sum^n_(i=1) (X_i - overline(X))^2) / (n - 1) $

\
=== Standard Deviation (Population & Sample)

Population: $ sigma = sqrt((sum^N_(i=1) (X_i - mu)^2) / N) $
Sample: $ s = sqrt((sum^n_(i=1) (X_i - overline(X))^2) / (n - 1)) $

\
=== Semideviation (Downside) Deviation

Remove all observations where $X_i > overline(X)$

$ sigma = sqrt((sum^N_(i=1) (X_i - mu)^2) / N) $

where $X_i <= overline(X)$

\
=== Coefficient of Variation ($C_V$)

The coefficient of variation measures the amount of risk ($sigma$) per unit of mean return ($mu$), or the measure of _risk-to-return_.

$ C_V = sigma / mu $

Or, for #underline("sample") observations

$ C_V = s / overline(X) $

\
=== Sharpe Ratio 

The Sharpe Ratio is a common measurement for _return-to-risk_ ($1/C_V$) with an adjusted mean return in excess of a risk-free rate. The larger the Sharpe ratio, the more attractive an asset's returns are to an investor, all else equal.

$ S_r = (mu_p - r_f) / sigma_p $

where

$mu_p = "Geometric Mean of Portfolio Returns"$

$r_f = "Risk-free Rate"$

$sigma_p = "Standard Deviation of Portfolio Returns "$

\
=== Standard ("$z$") Scores

The $z$-score represents the distance between the raw score $x$ and the mean $mu$ in units of standard deviation. The $z$-score is negative when the raw score is below the mean, and postive when above the mean.

$ z = (x - mu) / sigma $

\
=== Skewness


$ S_K = [n / ((n-1)(n-2))] (sum^n_(i=1)(X_i - overline(X))^3) / s^3 $

Or,

$ S_K approx (1/n) (sum^n_(i=1)(X_i - overline(X))^3) / s^3 $

when $n -> infinity$

\
=== Sample Excess Kurtosis

$K_E = n(n+1) / ((n-1)(n-2)(n-3)) (sum^n_(i=1)(X_i - overline(X))^4) / s^4 - (3(n-1)^2) / ((n-2)(n-3))$

Or,

$ K approx 1 / n (sum^n_(i=1)(X_i - overline(X))^4) / S^4 - 3 $

when $n -> infinity$

== *Probability Theory*
\
=== Definition of a Probability

- The probability of an event $P(E)$ is between 0 and 1

$ 0 <= P(E) <= 1 $

- The sum of probabilities for any mutually exclusive and exhaustive set of events is equal to 1

$ sum^E_(i=0) P(E_i) = 1 $

where

$P(E_i union E_(i+1)) = P(E_i) + P(E_(i+1))$,

$P(E_i sect E_(i+1)) = 0$

\
=== Unconditional (Marginal) Probability

An unconditional probability is the probability of an independent event occuring.

$ P(A) $
\
=== Conditional Probability

A conditional probability is the probability of an event occuring given another event has occured.

$ P(A|B) $

where 

$P(A|B) = P("AB") / P(B)$, $P(B) != 0$

\
=== Multiplicative Probability

$ P("AB") = P(A|B) P(B) = P(A) P(B)$

\
=== Additive Probability

$ P("A or B") = P(A) + P(B) - P("AB") $

\
=== Independent Events

Events A and B are independent, iff:

$ P(A|B) = P(A) or P(B|A) = P(B) $

\
=== Expected Value $E(X)$

The expected value $E(X)$ of a random variable $X$ is the probability weighted average of the observed outcomes of the random variable.

$ E(X) = sum^n_(i=1) P(X_i)X_i $

\
=== Variance of a Random Variable

The variance of a random variable is the expected value $E(X)$ of the squared deviations from the random variable's expected value.

$ sigma^2(X) = "Var"(X) = sum^n_(i=1) P(X_i)[X_i - E(X)]^2 $

\
=== Standard Deviation of a Random Variable

$ sigma(X) = sqrt(sum^n_(i=1) P(X_i)[X_i - E(X)]^2) $

\
=== Conditional Expected Values

$ E(X|S) = sum^n_(i=1) P(X_i | S)X_i $

\
=== Total Probability Rule for Expected Value

Converts a conditional probability into an unconditional probability.

$ E(X) = sum^n_(i=1) E(X_i | S_i)P(S_i) $

\
=== Expected Portfolio Return

$ "ER"_p = E(sum_(i=1)w_i R_i) $

\
=== Covariance of Portfolio Holding Returns

$ sigma(R_A, R_B) = EE[(R_A - "ER"_A)(R_B - "ER"_B)] $

Or,

\

$sigma(R_A, R_B) = sum^n_(i=1) sum^n_(j=1) P(R_A_i, R_B_j) (R_A_i - "ER"_A) (R_B_j - "ER"_B)$

\

\
=== Expected Portfolio Variance

$ sigma^2(R_p) = EE{[R_p - "ER"_p]^2} $

Or,

$sigma^2(R_p) = sum^n_(i=1) sum^n_(j=1) w_i w_j sigma(R_i, R_j)$


\
=== Pearson's Correlation Coeffecient

For a population:

$ rho(R_A, R_B) = EE[(R_A - "ER"_A)(R_B - "ER"_B)] / ((sigma R_A)(sigma R_B)) $


Or, for a sample:

$r_(R_A R_B) = (sum^n_(i=1) (R_A - "ER"_A)(R_B - "ER"_B)) / (sqrt(sum^n_(i=1) (R_A - "ER"_A)^2) sqrt(sum^n_(i=1) (R_B - "ER"_B)^2))$

where

$R_A and R_B = "Random Variables"$

$"ER"_A and "ER"_B$ = Probability weighted average expected values of random variables


\
=== Baye's Forumla

$P(E|I)$ can be expressed as the probability of new information given an event, divided by the unconditional probability of the new information, times the prior probability of the event. Baye's theorem uses the occurence of an event to infer the probability of the scenario generating it, and therefore is often described as an inverse probability.

\

$ P(E|I) = P(I|E) / P(I) dot P(E) $

where

$E = "Event"$

$I = "New Information"$

\
=== Combinations (Binomial) and Permutations
\

A combinatorial is the number of ways to choose $r$ options from a total of $n$ options, when the order in which the $r$ options are listed _does not_ matter.

$ attach(C, bl: n, br: r) = binom(n, r) = n! / ((n - r)!r!) $
\

A permutation is the number of ways to choose $r$ options from a total of $n$ options, when the order in which the $r$ options are listed _does_ matter.

$ attach(P, bl: n, br: r) = n! / (n - r)! $
\

\
=== Binomial (Discrete Random) Distributions

Binomial Random Variable $ X ~ B(n, p) $

Bernoulli Random Variable $ Y ~ B(1, p) $

where

$p$ = The probability of success (constant for all trials)

$n$ = The number of trials (or occurences)

The probability of $x$ successes in $n$ trials is calculated as:

$ p(x) = n! / ((n - x)!x!) p^x (1-p)^(n-x) $

\
=== Continuous Distributions




// #canvas({
//   import draw: *
//   // set-style(axes: (tick: (length: -.05)))
//   plot.plot(size: (3,3), x-tick-step: 1, axis-style: "left", {
  
//   let prices = (1, 2, 3, 4, 5, 6, 7, 8, 9)

//   for i in range(0, prices.len()) {
//     plot.add(domain: (-5, 5),
//       x => x + i,
//       fill: true, style: palette.tango)
//     } })  
// })