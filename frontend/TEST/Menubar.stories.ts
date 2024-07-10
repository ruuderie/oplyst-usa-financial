import type { Meta, StoryObj } from '@storybook/vue3';

import Menubar from '../components/ui/menubar/Menubar.vue';

const meta = {
  title: 'Menubar',
  component: Menubar,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Menubar>;

export default meta;
type Story = StoryObj<typeof Menubar>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};