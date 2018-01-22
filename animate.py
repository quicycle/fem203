'''
Animate output from fem203.

TODO:
- Work out how to visualise 2/3/4D simulations!


NOTE: This is not yet functional (taken from fdtd.py)
'''
import sys
import json
import argparse

from collections import OrderedDict

import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt

from matplotlib import animation


# .: Plot configuration :.
sns.set(style='white')
sns.set_context('notebook', font_scale=1.5)
gruvbox = [
    '#cc241d', '#fabd2f', '#458588', '#b16286',
    '#689d6a', '#fe8019', '#7c6f64', '#504945'
]
sns.set_palette(gruvbox)


class SimulationRenderer:
    '''
    NOTE: The handling of device regions is only going to work in the 1D
          case using this format. (See sample_output.json)
          -> Need to work out a generic format that allows for specifying
             a region in nD.
    '''
    def __init__(self, fname):
        try:
            data = json.loads(open(args.fname, 'r'))
        except:
            print("Must provide a valid file path", file=sys.stderr)

        self.title = data['title']
        self.grid_size = data['grid-size']
        self.n_steps = data['n-steps']
        self.ep_r = data['epsilon-r']
        self.mu_r = data['mu-r']
        self.devices = data['devices']

        # Dicts of `name: [[...], ...]`. (One row per simulation step)
        # Using OrderedDict to ensure we get the same iteration order
        self.e_fields = OrderedDict(data['fields']['E'])
        self.b_fields = OrderedDict(data['fields']['B'])

    def initialise_plot(self, figsize, dpi):
        '''
        Set up the animation figure, axes and lines
        '''
        x, y = figsize
        fig = plt.figure(figsize=(x/dpi, y/dpi), dpi=dpi)
        ax = plt.axes(xlim=(0, self.grid_size-2), ylim=(-2, 2))

        ax.set_title(self.title)
        ax.legend(prop=dict(size=10))

        # Shade device regions
        for device in self.devices:
            ε, μ = device['er'], device['mr']
            start, stop = device['start'], device['stop']
            if ε == μ:
                color = 'grey'
            elif ε > μ:
                color = 'yellow'
            else:
                color = 'red'
            if ε * μ > self.ep_r * self.mu_r:
                alpha = 0.5
            else:
                alpha = 0.3
            ax.axvspan(start, stop, alpha=alpha, color=color)

        x = np.arange(0, self.grid_size)

        lines = []

        for ix, (label, field) in enumerate(self.e_fields.items()):
            lines.append(ax.plot(x, field[0], label=label, lw=2)[0])
            # Initialise to current values
            lines[ix].set_data(x, field[0])

        for ix, (label, field) in enumerate(self.b_fields.items()):
            lines.append(ax.plot(x, field[0], label=label, lw=2)[0])
            # Initialise to current values
            lines[len(self.e_fields)+ix].set_data(x, field[0])

        return fig, ax, lines, x

    def run(self, figsize, dpi, output_fname=None, output_file_type='mp4'):
        '''
        Add in Electric and Magnetic sources then run the simulation.
        If a filename is specified then output will be saved under that name
        in the working directory.
        '''
        fig, ax, lines, x = self.initialise_plot(figsize, dpi)

        def init():
            '''Set the legend and return the initial plot'''
            plt.legend(handles=lines)
            return lines

        def animate(step):
            '''Set the field values to the next interval'''
            for ix, (_, field) in enumerate(self.e_fields.items()):
                lines[ix].set_data(x, field[step])

            for ix, (_, field) in enumerate(self.b_fields.items()):
                lines[len(self.e_fields)+ix].set_data(x, field[step])

            return lines

        anim = animation.FuncAnimation(
                fig, animate, init_func=init,
                frames=self.n_steps, interval=0.1)

        if output_fname:
            if output_file_type == 'mp4':
                anim.save(
                    '{}'.format(output_fname), writer='ffmpeg',
                    fps=100, bitrate=2000)
            elif output_file_type == 'gif':
                anim.save(
                    '{}'.format(output_fname), writer='imagemagick',
                    fps=100, dpi=dpi)
            else:
                raise ValueError('filename must end in either .mp4 or .gif')

        sns.set_context('notebook', font_scale=2)
        plt.show()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'fname',
        required=True
    )
    parser.add_argument(
        '-o',
        '--output-fname',
        default=None
    )
    parser.add_argument(
        '-t',
        '--output-file-type',
        default='mp4',
        help='One of "mp4" or "gif"'
    )
    parser.add_argument(
        '--dpi',
        default=50,
        help='Resolution of the video'
    )
    parser.add_argument(
        '--figsize',
        default='500x300',
        help='Figure size in "XxY" format. i.e. 500x300'
    )

    args = parser.parse_args()

    try:
        figsize = tuple(map(int, args.figsize.split('x')))
    except ValueError:
        raise RuntimeError('Invalid figsize: %s' % args.figsize)

    s = SimulationRenderer(args.fname)
    s.run(figsize, args.dpi, args.output_fname, args.output_file_type)
