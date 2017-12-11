DIR = resources
SRC = chars.ans extra.ans oozz.ans

LATIN1 = $(SRC:%.ans=$(DIR)/%.latin1)
	
all: $(LATIN1)

$(DIR)/%.latin1: $(DIR)/%.ans
	# recode 437 < $< > $@
	recode 437 < $< | sed -e 's/\[37m/\[39m/g' > $@

