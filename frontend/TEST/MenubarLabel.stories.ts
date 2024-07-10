import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarLabel from '../components/ui/menubar/MenubarLabel.vue';

const meta = {
  title: 'MenubarLabel',
  component: MenubarLabel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarLabel>;

export default meta;
type Story = StoryObj<typeof MenubarLabel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};