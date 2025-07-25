from random import randint
from subprocess import PIPE
from subprocess import Popen
from threading import Thread
from tkinter import *
from tkinter import scrolledtext


def run():
    """运行程序"""
    root = Tk()
    root.resizable(False, False)
    root.title("Vinegar")
    font_cfg = ("微软雅黑", 11)

    # 编码相关
    def enc():
        """执行加密操作"""
        info.config(state='normal')
        info.delete('1.0', END)
        info.config(state='disabled')
        text = plain.get('1.0', END).rstrip()
        param = param_a.get() * 1000 + param_b.get() * 10 + param_c.get()
        keyword = key_txt.get('1.0', END).rstrip()
        process = Popen([".\\Vinegar", "encode"],
                        stdout=PIPE, stdin=PIPE)
        text = process.communicate(input=text.encode('utf-8'))[0].decode(
            'utf-8')
        cipher.delete('1.0', END)
        cipher.insert('1.0', text)

    def dec():
        """执行解密操作"""
        info.config(state='normal')
        info.delete('1.0', END)
        info.config(state='disabled')
        text = cipher.get('1.0', END).rstrip()
        param = param_a.get() * 1000 + param_b.get() * 10 + param_c.get()
        keyword = key_txt.get('1.0', END).rstrip()
        process = Popen([".\\Vinegar", "decode"],
                        stdout=PIPE, stdin=PIPE)
        text = process.communicate(input=text.encode('utf-8'))[0].decode(
            'utf-8')
        plain.delete('1.0', END)
        plain.insert('1.0', text)

    # 菜单栏
    menu_bar = Menu(root, font=font_cfg)

    # 编辑菜单
    def undo():
        """撤销"""
        try:
            plain.edit_undo()
        except TclError:
            pass
        try:
            cipher.edit_undo()
        except TclError:
            pass

    def redo():
        try:
            plain.edit_redo()
        except TclError:
            pass
        try:
            cipher.edit_redo()
        except TclError:
            pass

    def clear():
        """清空屏幕上的文本"""
        plain.delete('1.0', END)
        cipher.delete('1.0', END)
        info.config(state='normal')
        info.delete('1.0', END)
        info.config(state='disabled')

    def swap():
        """交换明文与密文位置"""
        new_plain = cipher.get('1.0', END)
        new_plain = new_plain.rstrip()
        new_cipher = plain.get('1.0', END)
        new_cipher = new_cipher.rstrip()
        plain.delete('1.0', END)
        plain.insert(INSERT, new_plain)
        cipher.delete('1.0', END)
        cipher.insert(INSERT, new_cipher)

    edit_menu = Menu(menu_bar)
    edit_menu.add_command(label="撤销", command=undo)
    edit_menu.add_command(label="重做", command=redo)
    edit_menu.add_command(label="清空", command=clear)
    edit_menu.add_command(label="交换明文与密文", command=swap)
    menu_bar.add_cascade(label="编辑", menu=edit_menu)

    def popupmenu(event):
        """右键弹出编辑菜单"""
        edit_menu.post(event.x_root, event.y_root)

    root.bind("<Button-3>", popupmenu)

    root.config(menu=menu_bar)

    # 明文栏
    Label(root, text="明文",
          font=font_cfg, ). \
        grid(row=0, column=0)
    plain = scrolledtext.ScrolledText(root, undo=True,
                                      width=25, height=20,
                                      font=font_cfg)
    plain.grid(row=1, rowspan=20, column=0)
    # 密文栏
    Label(root, text="密文",
          font=font_cfg). \
        grid(row=0, column=2)
    cipher = scrolledtext.ScrolledText(root, undo=True,
                                       width=25, height=20,
                                       font=font_cfg)
    cipher.grid(row=1, rowspan=20, column=2)
    # 加密按钮
    Button(root, text="加密",
           width=10, height=1,
           font=font_cfg,
           command=lambda: create_thread(enc)). \
        grid(row=1, column=1, sticky=N + S)
    # 解密按钮
    Button(root, text="解密",
           width=10, height=1,
           font=font_cfg,
           command=lambda: create_thread(dec)). \
        grid(row=2, column=1, sticky=N + S)
    # 清空按钮
    Button(root, text="清空",
           width=10, height=1,
           font=font_cfg,
           command=clear). \
        grid(row=3, column=1, sticky=N + S)
    # 信息栏
    info = Text(root, undo=True,
                width=66, height=2,
                font=font_cfg)
    info.grid(row=21, column=0, columnspan=3, sticky=S)
    info.insert(INSERT, "请输入需要加密或解密的文本...")
    info.config(state='disabled')

    # 加密设置相关
    Label(root, text="加密设置",
          font=font_cfg). \
        grid(row=0, column=3, columnspan=2)

    # 加密参数A控制
    Label(root, text="加密参数A",
          font=font_cfg). \
        grid(row=1, column=3)
    param_a = Scale(root, from_=1, to=9,
                    font=font_cfg,
                    length=font_cfg[1] * 16,
                    orient=HORIZONTAL)
    param_a.grid(row=1, column=4, sticky=W)
    param_a.set(randint(1, 9), )
    # 加密参数B控制
    Label(root, text="加密参数B",
          font=font_cfg). \
        grid(row=2, column=3)
    param_b = Scale(root, from_=1, to=63,
                    font=font_cfg,
                    length=font_cfg[1] * 16,
                    orient=HORIZONTAL)
    param_b.grid(row=2, column=4, sticky=W)
    param_b.set(randint(1, 63))
    # 加密参数C控制
    Label(root, text="加密参数C",
          font=font_cfg). \
        grid(row=3, column=3)
    param_c = Scale(root, from_=2, to=9,
                    font=font_cfg,
                    length=font_cfg[1] * 16,
                    orient=HORIZONTAL)
    param_c.grid(row=3, column=4, sticky=W)
    param_c.set(randint(2, 9))
    # 关键词
    Label(root, text="关键词",
          font=font_cfg). \
        grid(row=4, column=3)
    key_txt = Text(root, undo=True,
                   width=font_cfg[1] * 5 // 3, height=5,
                   font=font_cfg)
    key_txt.grid(row=4, column=4, sticky=W)

    mainloop()


def create_thread(func):
    """创建线程:
       func: 函数名"""
    t = Thread(target=func, daemon=True)
    t.start()


run()
