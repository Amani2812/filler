# Makefile for Filler AI Player

NAME = filler

all: $(NAME)

$(NAME):
	cargo build --release
	cp target/release/$(NAME) ./$(NAME)

clean:
	cargo clean
	rm -f $(NAME)

fclean: clean

re: fclean all

test: $(NAME)
	@echo "Build successful! Binary ready at: target/release/$(NAME)"

.PHONY: all clean fclean re test
