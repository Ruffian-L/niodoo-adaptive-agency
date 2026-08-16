# Calibration and excluded coordinates

Several setup coordinates preceded the accepted sweep and are not silently counted as
memory-capacity results:

| coordinate | result | why excluded |
|---|---|---|
| initial empty result directory | no result | runner did not complete |
| early K=0/K=1/K=2 attempts | fail or incomplete | the historical flag configuration had not yet been restored |
| long-output K=0 attempt | fail at 1,536 tokens | wrong route configuration |
| disclosed two-turn K=0 attempt | fail at 498 tokens | extra human text changed the sealed prompt |
| single-turn undisclosed K=0 calibration | exact pass at 350 tokens | authorized exception used only to establish that the harness reproduced the flag |

After calibration, the evaluation notice was placed in every test store as a constant
entry. This preserved Jason's commitment to tell Nex when it was being evaluated while
keeping the task prompt byte-identical. Only the accepted runs in `results.csv` support
the durability conclusions.
