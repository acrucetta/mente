

![https://drek4537l1klr.cloudfront.net/chollet2/Figures/01-02.png](https://drek4537l1klr.cloudfront.net/chollet2/Figures/01-02.png)

Unlike statistics, machine learning deals with large and complex datasets. It is fundamentally an engineering discipline. Unlike theoretical physics or mathematics it is a very hands on field driven by empirical findings and deeply reliant on advances in software and hardware.

The central problem is how can we meaningfully transform data, in other words, learn useful representations of the input at hand to get us closer to the expected output.

## Videos

Source: https://www.youtube.com/watch?v=zjkBMFhNj_g

- Little is known in full detail
	- Billions of parameter collaborate, then we adjust them, we don't really know how the billion of parameters collaborate
	- They're inscrutable artifacts

- How to train a model
	- Pretrain
		- Download 10TB of text
		- Get a cluster of GPUs
		- Compress the text into a NN, pay $2MM, wait 12 days
		- Obtain **base model**
	- Finetuning
		- Write labeling instructions
		- Hire people, collect responses (scale AI)
		- Fine-tune base model, wait ~1 day
		- Obtain assistant model
		- Run lots of evals
		- Deploy
		- Monitor, **go to step 1**

- Scaling laws
	- N - number of parameters
	- D - amount of text
	- We can expect more intelligence for free by scaling
	- We can expect a lot more "general capability" across all areas of knowledge

## Papers

### Scaling Laws

![[Pasted image 20241008210949.png]]

- 