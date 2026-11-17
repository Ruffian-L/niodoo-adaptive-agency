# PARB compare card

- when: 2026-08-17T20:25:05.081124+00:00
- model: `/home/ruffianl/projects/niodoo-live/model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf` sha256=`14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7`
- llama-cli: `/home/ruffianl/projects/llama.cpp-sol-glimmer/build-sm121/bin/llama-cli`
- niodoo binary sha256: `80b4b95c23f9210600c474dad376353ce09045c3d3410f6ce1589f51fb7d7dc2`
- seed: 42
- scorer: `/media/ruffianl/ghost_team/02_projects/projects/niodoo-parb-physics/harness/parb/scorer.py`
- transcripts: `/media/ruffianl/ghost_team/02_projects/projects/niodoo-parb-physics/runs/parb/niodoo_iter36_b152`
- llama_total_correct: 24
- niodoo_total_correct: 25

| id | gold | llama | niodoo | L | N |
|----|------|-------|--------|---|---|
| PHYS_001 | 1 hour | it will take 50 hours to dry 50 towels. | it would take 50 hours to dry 50 towels. | False | False |
| LOGIC_001 | 5 cents | the ball costs $ 0.05. | the ball costs $ 0.05 or 5 cents. | True | True |
| PHYS_002 | On the table | The diamond is still inside the cup. | Since you put it in the cup, flipped the cup, an | False | False |
| LOGIC_002 | 1 sister | There are 4 brothers, so if we multiply the numb | Alice has 8 sisters. | False | False |
| AMBIG_001 | The trophy | Is it the trophy thats too large, or is the suit | The trophy is too large. | False | True |
| AMBIG_002 | The suitcase | The suitcase cannot fit the trophy because it is | The brown suitcase is too small. | False | False |
| PHYS_003 | same speed | in a vacuum chamber on Earth, a 10kg lead brick  | Since they are both on the surface of Earth at t | False | False |
| LOGIC_003 | Yes | switching from Door 1 to Door 2 will give you th | Am I correct? | False | False |
| SEMANTIC_002 | 3 | There are 2 rs in the word strawberry. | There are 2 rs in the word strawberry. | False | False |
| LOGIC_004 | Yes | yes, a married person (Jack) is looking at an un | Am I correct? | True | False |
| PHYS_004 | Add it now | wait 5 minutes and then add the cold milk. | This will allow the coffee to reach its optimal  | False | False |
| LOGIC_005 | 9 | the farmer has 8 sheep left. | 8. | False | False |
| PHYS_005 | Right hand | If your left hand is holding the book in reality | This is because the mirror is reversing the imag | False | False |
| LOGIC_006 | 5 minutes | 5 minutes. | The number of machines and the number of widgets | True | False |
| CAUSAL_001 | No | In this scenario, if | If the | False | False |
| LOGIC_007 | no missing dollar | - Manager returned | the Manager | False | False |
| PHYS_006 | No | **Specialized equipment**: In some cases, scient | **Water-specific techniques**: Some athletes, su | False | False |
| SEMANTIC_003 | equal | Since both are one pound, a pound of lead and a  | in terms of weight, theyre equal. | False | True |
| LOGIC_008 | 312211 | the next number in the sequence is 312211. | the next number in the sequence is 312211. | True | True |
| SEMANTIC_004 | None | He is famous for leading the Israelites out of s | to answer your question, Moses did not take any  | False | False |
| PHYS_008 | Backward | This is an example of Newtons first law of motio | when the car hits the brakes, the car begins to  | False | False |
| LOGIC_009 | don't bury survivors | they dont bury the survivors. | The survivors dont get buried. | False | False |
| MATH_002 | 70 | 70. | 60 + 10 = 70 | True | True |
| PHYS_009 | same | But this effect is usually minimal and can be ig | the water level does not rise or fall. | False | False |
| LOGIC_010 | 1 hour | the pills will last for 16 hours. | 3 pills will last 1.5 hours, as you will take on | False | False |
| SEMANTIC_005 | Frankfort | As for the pronunciation of Louisville, its gene | The correct pronunciation of Louisville is Loo-e | False | False |
| PHYS_010 | Upside down | The shape of the concave surface allows it to co | is: upside down | False | True |
| LOGIC_011 | quarter and a nickel | the two coins are a nickel (5 cents) and a quart | A quarter and a nickel. | False | True |
| CAUSAL_002 | Sunday | The day after tomorrow is Sunday. | if yesterday was tomorrow, today would be... | True | False |
| SEMANTIC_006 | banana | Thats a classic joke, and its often used as a pu | ...piece of fruit! | False | False |
| LOGIC_012 | Second | you would be in second place, but you are about  | I am now in second place. | False | True |
| PHYS_011 | Hotter | Eventually, the room will reach a temperature hi | If you leave the refrigerator door open in a per | True | True |
| LOGIC_013 | Daughter | However, I can give you a more direct answer to  | If Teresas daughter is my daughters mother, that | False | True |
| SEMANTIC_007 | Red | while the color of the blood itself might appear | However, as venous blood flows through the lungs | True | True |
| PHYS_012 | Yes | while it may seem like a shadow can move faster  | Let me know if you have any other questions or i | False | False |
| LOGIC_014 | 12 | December | : All of them. | False | False |
| LOGIC_015 | 1 hour | you get 11 hours of sleep. | 13 hours | True | False |
| SEMANTIC_008 | Neither | Even though its part of a plural phrase (the egg | The correct sentence is in the singular form, is | False | False |
| LOGIC_016 | paradox | However, in informal discussions, its common to  | to summarize, the sentence This sentence is fals | True | False |
| PHYS_013 | Yes | to answer your question: the light from your hea | is: $ \boxed{yes} $ | False | True |
| LOGIC_017 | 13/27 | the probability of the second child being a boy  | the probability that the second child is also a  | False | False |
| AMBIG_003 | The rope | Based on the information provided, it seems that | The rope was too weak. | True | True |
| PHYS_014 | front to back | On the other hand, the up-down | Your left and right sides are swapped, but your  | False | False |
| LOGIC_018 | penny | A penny has a head on one side and a tail on the | A penny. | True | True |
| CAUSAL_003 | Monopoly | In this case, the owner might be assisting the m | The man parks his car. | False | False |
| SEMANTIC_009 | 12 | All 12 months have at least 28 days. | Twelve months have 28 days. | True | True |
| PHYS_015 | same time | they will hit the ground at the same time. | neither the dropped bullet nor the fired bullet  | True | False |
| AMBIG_004 | councilmen | The city councilmen feared violence. | The city councilmen feared violence. | True | True |
| PHYS_016 | Mpemba | **Convection**: Hot water conducts heat more eff | This phenomenon was first observed by a Tanzania | False | False |
| SEMANTIC_010 | Incorrectly | incorrectly. | incorrectly. | True | True |
| PHYS_017 | Yes | This concept has far-reaching implications for o | If we were to remove the measuring device, the p | False | False |
| LOGIC_019 | 1 hour | the pills would last for 1.5 hours. | 30 minutes, 1 hour, 1.5 hours, 2 hours, 2.5 hour | False | False |
| SEMANTIC_011 | newspaper | Newspapers are traditionally printed with black  | a newspaper. | True | True |
| PHYS_018 | Both | In other words, | the combination of Bernoullis principle and Newt | False | False |
| LOGIC_020 | chicken first | This solution ensures that the fox and the chick | Take the fox across the river and leave it on th | False | False |
| PHYS_019 | Slower | As a result, the magnet will experience a slight | fall slower than it would if it were falling thr | True | True |
| SEMANTIC_012 | stamp | A stamp can travel around the world while stayin | : A stamp. | True | True |
| AMBIG_005 | testimony | Could you provide more context or clarify who te | There is not enough information in the text to s | False | False |
| LOGIC_021 | 2/3 | the probability that the other child is a girl i | is: $ \boxed{\frac{1}{2}} $ | False | False |
| PHYS_020 | Rayleigh scattering | **Aurorae**: In some regions | The darkness is a result of the absence of direc | False | False |
| CAUSAL_004 | ice | In this scenario, the man may have been exposed  | Do you have any other questions or would you lik | False | False |
| SEMANTIC_013 | Glass | greenhouses are made of glass. | : Green bricks! | True | False |
| LOGIC_022 | Once | you can subtract 10 from 100 exactly 10 times. | infinitely many! | False | False |
| PHYS_021 | No | **On a celestial body with a strong gravitationa | to answer your question: no, a ball thrown in sp | False | True |
| LOGIC_023 | piano | A piano has keys but it cant open locks. | Piano | True | True |
| PHYS_022 | No | These devices can efficiently cool a larger area | However, its essential to note that this effect  | False | False |
| SEMANTIC_014 | Everest | Mount Everest was not officially discovered unti | Over time, further measurements and surveys have | True | True |
| LOGIC_024 | 12 | there are 6 fish left. | 12! | False | True |
| PHYS_023 | Yes | So | **Philosophical or metaphorical interpretations: | False | False |
| SEMANTIC_015 | 11 | There are 26 letters in the alphabet. | There are 26 letters in the modern English alpha | False | False |
| LOGIC_025 | S | the next month abbreviation after J J A would be | December | True | False |
| PHYS_024 | No for you | For example, if you | This effect, known as time dilation, only become | False | False |
| PHYS_025 | 1 hour | 10 candles will burn for 10 hours if you light t | 0 hours, as all 10 candles will extinguish each  | False | False |
| LOGIC_026 | You | If youd like to provide more context or informat | : you! | True | True |
| SEMANTIC_016 | All of them | All 12 months have at least 28 days. | All 12 months have at least 28 days | False | False |
| SEMANTIC_017 | Wrong | Many people pronounce it as nu-klee-er, but the  | Nuclear | False | False |
| PHYS_026 | No smoke | the direction of the smoke will be roughly north | The smoke will blow south. | False | False |
