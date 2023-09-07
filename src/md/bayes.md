## Bayes' Probability

*Take evidence and account for false positives*

### Formula

![bayes illustrated](https://betterexplained.com/ColorizedMath/img/Bayes_Theorem.png)

- Pr(H|E) = Chance of having cancer (H) given a positive test (E). This is what we want to know: How likely is it to have cancer with a positive result? In our case it was 7.8%.
- Pr(E|H) = Chance of a positive test (E) given that you had cancer (H). This is the chance of a true positive, 80% in our case.
- Pr(H) = Chance of having cancer (1%).
- Pr(not H) = Chance of not having cancer (99%).
- Pr(E|not H) = Chance of a positive test (E) given that you didn’t have cancer (not H). This is a false positive, 9.6% in our case.

It comes down to the change of a **true positive** divided by the change of any **positive.**

![](https://betterexplained.com/wp-content/plugins/wp-latexrender/pictures/abb9243a6a7d838ec02cc5f9713e5604.png)

It summarizes to the change Pr(E) of getting any positive result, whether a true positive in the cancer population or a false positive in the non-cancer population. (A weighting factor)

When we normalize for false positives we get the very low chance of cancer given a positive test.

