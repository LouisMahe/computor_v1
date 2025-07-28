NAME = libpolylib.a
CC = clang
CFLAGS = -Werror -Wall -Wextra -O2
SRCS = polylib.c
OBJS = $(SRCS:%.c=%.o)

all:	$(NAME)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(NAME): $(OBJS)
	ar crs $(NAME) $(OBJS)
	rm -f $(OBJS)

clean:
	rm -f $(OBJS) $(NAME)

re:
	clean
	all