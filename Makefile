DIR = resources
SRC = chars.ans extra.ans

LATIN1 = $(SRC:%.ans=$(DIR)/%.latin1)
	
all: $(LATIN1)

$(DIR)/%.latin1: $(DIR)/%.ans
	recode 437 < $< > $@

