resources/%.latin1: resources/%.ans
	recode 437 < $< > $@

