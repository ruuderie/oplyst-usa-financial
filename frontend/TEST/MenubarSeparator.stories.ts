import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarSeparator from '../components/ui/menubar/MenubarSeparator.vue';

const meta = {
  title: 'MenubarSeparator',
  component: MenubarSeparator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarSeparator>;

export default meta;
type Story = StoryObj<typeof MenubarSeparator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};