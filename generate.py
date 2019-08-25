import glob
import os
import markdown2 as md
import datetime as dt
from rfeed import *

ROOT_URL = "https://goncalopalaio.github.io/"
URL_TO_FEED = ROOT_URL + "rss"

SEP = "_"
POST_PREFIX = "post_"
EXTENSION_HTML = ".html"
EXTENSION_MD = ".md"
TEMPLATE_EXTENSION_HTML = ".template.html"
TEMPLATE_POST_REPLACEMENT = "<!-- _POSTS_ -->"
TEMPLATE_POST_CONTENT_REPLACEMENT = "<!-- _POST_CONTENT_ -->"
TEMPLATE_TITLE_REPLACEMENT = "<!-- _TITLE_ -->"


def log_list(text, lst):
	print(text)
	for i in lst:
		print("\t%s" % str(i))


def read_file_contents(path):
	with open(path) as f:
		return "".join(f.readlines())


def write_content_to_file(path, content):
	with open(path, "w") as f:
		f.write(content)
		f.write('\n')


def find_files(root, wildcard):
	files = [f for f in glob.glob(root + os.path.join(wildcard))]
	return files


def replace_post_list(template_file, posts_links):
	destination_file = template_file.replace(TEMPLATE_EXTENSION_HTML, EXTENSION_HTML)
	with open(destination_file, "w") as destination:
		with open(template_file) as template:

			for line in template:
				if line.strip() == TEMPLATE_POST_REPLACEMENT:
					for link in posts_links:
						destination.write(link)
				else:
					destination.write(line)


def parse_post_title(line):
	first_10_lines = min(10, len(line))

	for i in range(first_10_lines):
		current_line = line[i]
		if current_line.startswith("# "):
			return current_line.replace("# ", "")

	return "Untitled"


def parse_post_date(file):
	date_in_file_name = file.split(SEP)
	try:
		date = dt.datetime.strptime(date_in_file_name[1], '%d%m%Y')
		return date, date.strftime(' - %d/%m/%Y')
	except Exception as e:
		print("Error: Could not get date from %s" % file)
		return None, ""


def create_link(title, path):
	return md.markdown("[%s](%s)" % (title, path))


def create_htmls_from_mds(post_template, file):
	with open(file) as f:
		lines = f.readlines()

		title = parse_post_title(lines)
		date, date_str = parse_post_date(file)

		dest = file.replace(EXTENSION_MD, EXTENSION_HTML)

		link = create_link(title + date_str, dest)

		content = "".join(lines)
		content = md.markdown(content)

		post_template = post_template.replace(TEMPLATE_TITLE_REPLACEMENT, title)
		
		print("Creating html from %s -> %s" % (file, dest))

		write_content_to_file(dest, post_template.replace(TEMPLATE_POST_CONTENT_REPLACEMENT, content))

		return dest, title, date, link


def generate_rss_file(posts_info):
	items_feed = []

	for file, title, date in posts_info:
		post_url = ROOT_URL + file
		print("Adding post to rss: %s" % post_url)
		item = Item(
			title = title,
			link = post_url, 
			description = title,
			author = "Gonçalo Palaio",
			guid = Guid(post_url),
			pubDate = date)
		items_feed.append(item)

	feed = Feed(
	title = "Gonçalo Palaio Blog",
	link = URL_TO_FEED,
	description = "Random musings about programming",
	language = "en-US",
	lastBuildDate = dt.datetime.now(),
	items = items_feed)

	write_content_to_file("rss", feed.rss())


def main():
	post_template = read_file_contents("post%s" % TEMPLATE_EXTENSION_HTML)

	if not post_template:
		print("failed to read post template. Stopping")
		return


	posts = find_files("", "%s*%s" % (POST_PREFIX, EXTENSION_MD))
	posts = sorted(posts, reverse = True)

	log_list("Found posts: ", posts)

	# Parse posts and convert them into html
	# Create html files for posts
	posts_info = []
	posts_links_html = []
	for p in posts:
		file, title, date, link = create_htmls_from_mds(post_template, p)
		posts_links_html.append(link)
		posts_info.append((file, title, date))


	# Insert list of posts in the index page
	templates = find_files("", "index%s" % TEMPLATE_EXTENSION_HTML)
	log_list("Found templates: ", templates)	
	for f in templates:
		replace_post_list(f, posts_links_html)

	generate_rss_file(posts_info)





if __name__ == '__main__':
	main()